use crate::{
    commands::{RecvCommand, SendCommand},
    traits::VmixTcpApiClient,
};
use anyhow::Result;
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

pub struct VmixApi {
    pub sender: SyncSender<SendCommand>,
    pub receiver: Receiver<RecvCommand>,
    shutdown_signal: Arc<AtomicBool>,
    error_signal: Arc<AtomicBool>, // New: shared error state
    original_stream: Arc<std::sync::Mutex<Option<TcpStream>>>, // New: keep original stream for explicit shutdown
    reader_handle: Option<JoinHandle<()>>,
    writer_handle: Option<JoinHandle<()>>,
}

// VmixApiはマルチスレッド環境で安全に使用できる
// 内部でMutexやAtomicBoolを使用しているため
unsafe impl Send for VmixApi {}
unsafe impl Sync for VmixApi {}

impl Drop for VmixApi {
    fn drop(&mut self) {
        // Signal all threads to shutdown immediately
        self.shutdown_signal.store(true, Ordering::SeqCst);
        self.error_signal.store(true, Ordering::SeqCst);
        
        // Explicitly close the original stream to force both reader and writer to exit
        // This will cause any blocking reads/writes to return with an error
        if let Ok(mut stream_guard) = self.original_stream.lock() {
            if let Some(stream) = stream_guard.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

        // Send a final message to wake up the writer thread if it's waiting
        let _ = self.sender.try_send(SendCommand::QUIT);

        // Wait for threads to complete with timeout
        let shutdown_timeout = Duration::from_millis(500);
        
        // Join reader thread with timeout
        if let Some(handle) = self.reader_handle.take() {
            // Give the reader thread a brief moment to notice the shutdown signal
            std::thread::sleep(Duration::from_millis(10));
            
            // Simple timeout: try to join, but don't wait forever
            let start_time = std::time::Instant::now();
            loop {
                if handle.is_finished() {
                    if handle.join().is_err() {
                        eprintln!("Warning: Reader thread panicked during shutdown");
                    }
                    break;
                }
                if start_time.elapsed() > shutdown_timeout {
                    eprintln!("Warning: Reader thread did not shut down within timeout");
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        
        // Join writer thread with timeout  
        if let Some(handle) = self.writer_handle.take() {
            let start_time = std::time::Instant::now();
            loop {
                if handle.is_finished() {
                    if handle.join().is_err() {
                        eprintln!("Warning: Writer thread panicked during shutdown");
                    }
                    break;
                }
                if start_time.elapsed() > shutdown_timeout {
                    eprintln!("Warning: Writer thread did not shut down within timeout");
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

impl VmixApi {
    pub async fn new(remote: SocketAddr, timeout: Duration) -> Result<Self> {
        // Connect with proper error handling
        let stream = TcpStream::connect_timeout(&remote, timeout)
            .map_err(|e| anyhow::anyhow!("Failed to connect to {}: {}", remote, e))?;

        stream
            .set_read_timeout(Some(timeout))
            .map_err(|e| anyhow::anyhow!("Failed to set read timeout: {}", e))?;

        stream
            .set_write_timeout(Some(timeout))
            .map_err(|e| anyhow::anyhow!("Failed to set write timeout: {}", e))?;

        // Create stream clones for reader and writer
        let mut reader_stream = stream
            .try_clone()
            .map_err(|e| anyhow::anyhow!("Failed to clone stream for reader: {}", e))?;
        
        // Set reader stream to non-blocking mode for better shutdown handling
        reader_stream
            .set_nonblocking(true)
            .map_err(|e| anyhow::anyhow!("Failed to set non-blocking mode: {}", e))?;
        let writer_stream = stream
            .try_clone()
            .map_err(|e| anyhow::anyhow!("Failed to clone stream for writer: {}", e))?;

        // Shared signals for coordination
        let shutdown_signal = Arc::new(AtomicBool::new(false));
        let error_signal = Arc::new(AtomicBool::new(false));
        let original_stream = Arc::new(std::sync::Mutex::new(Some(stream)));

        // Reader thread setup
        let (reader_sender, reader_receiver): (SyncSender<RecvCommand>, Receiver<RecvCommand>) =
            std::sync::mpsc::sync_channel(1);

        let reader_shutdown = shutdown_signal.clone();
        let reader_error = error_signal.clone();
        let reader_handle = std::thread::spawn(move || {
            loop {
                // Check shutdown or error signals at the beginning of each iteration
                if reader_shutdown.load(Ordering::Relaxed) || reader_error.load(Ordering::Relaxed) {
                    break;
                }

                // Try to read from stream with timeout handling
                // Note: This may still block, but the stream will be closed from Drop impl
                match (&mut reader_stream).try_into() {
                    Ok(command) => {
                        if reader_sender.send(command).is_err() {
                            // Receiver dropped, exit thread
                            break;
                        }
                    }
                    Err(err) => {
                        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
                            match io_err.kind() {
                                std::io::ErrorKind::WouldBlock => {
                                    // Non-blocking operation would block, sleep briefly and retry
                                    std::thread::sleep(Duration::from_millis(10));
                                    continue;
                                }
                                std::io::ErrorKind::TimedOut => {
                                    // Timeout is expected, continue loop
                                    continue;
                                }
                                std::io::ErrorKind::ConnectionAborted
                                | std::io::ErrorKind::ConnectionReset
                                | std::io::ErrorKind::UnexpectedEof => {
                                    // Connection error - signal error to writer thread
                                    eprintln!("Connection closed by remote: {}", err);
                                    reader_error.store(true, Ordering::Relaxed);
                                    break;
                                }
                                _ => {
                                    // Other IO errors - signal error to writer thread
                                    eprintln!("IO error in reader thread: {}", err);
                                    reader_error.store(true, Ordering::Relaxed);
                                    break;
                                }
                            }
                        } else {
                            eprintln!("Failed to parse incoming packet: {}", err);
                            continue;
                        }
                    }
                }
            }
        });

        // Writer thread setup
        let (writer_sender, writer_receiver): (SyncSender<SendCommand>, Receiver<SendCommand>) =
            std::sync::mpsc::sync_channel(1);

        let writer_shutdown = shutdown_signal.clone();
        let writer_error = error_signal.clone();
        let writer_handle = std::thread::spawn(move || {
            let mut writer = writer_stream;

            loop {
                // Check shutdown or error signals
                if writer_shutdown.load(Ordering::Relaxed) || writer_error.load(Ordering::Relaxed) {
                    break;
                }

                // Try to receive with timeout to allow checking signals
                match writer_receiver.recv_timeout(Duration::from_millis(10)) {
                    Ok(command) => {
                        // Check for quit command
                        if matches!(command, SendCommand::QUIT) {
                            let bytes: Vec<u8> = command.into();
                            if writer.write_all(&bytes).is_err() {
                                writer_error.store(true, Ordering::Relaxed);
                            }
                            // Quit command processed, exit thread
                            break;
                        }

                        let bytes: Vec<u8> = command.into();
                        if writer.write_all(&bytes).is_err() {
                            eprintln!("Failed to write to stream");
                            writer_error.store(true, Ordering::Relaxed);
                            break;
                        }

                        if writer.flush().is_err() {
                            eprintln!("Failed to flush stream");
                            writer_error.store(true, Ordering::Relaxed);
                            break;
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Timeout, continue to check signals
                        continue;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        // Channel disconnected, exit thread
                        break;
                    }
                }
            }
        });

        Ok(Self {
            sender: writer_sender,
            receiver: reader_receiver,
            shutdown_signal,
            error_signal,
            original_stream,
            reader_handle: Some(reader_handle),
            writer_handle: Some(writer_handle),
        })
    }

    /// Send a command to vMix
    pub fn send_command(&self, command: SendCommand) -> Result<()> {
        self.sender
            .send(command)
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))
    }

    /// Try to receive a command from vMix with timeout
    pub fn try_receive_command(&self, timeout: Duration) -> Result<RecvCommand> {
        self.receiver
            .recv_timeout(timeout)
            .map_err(|e| anyhow::anyhow!("Failed to receive command: {}", e))
    }

    /// Gracefully disconnect from vMix
    pub fn disconnect(&self) -> Result<()> {
        self.send_command(SendCommand::QUIT)
    }

    /// Check if the connection is still alive
    pub fn is_connected(&self) -> bool {
        // First check atomic flags for immediate shutdown/error detection
        if self.shutdown_signal.load(Ordering::Relaxed) || self.error_signal.load(Ordering::Relaxed) {
            return false;
        }
        
        // Perform lightweight socket health check
        if let Ok(stream_guard) = self.original_stream.try_lock() {
            if let Some(stream) = stream_guard.as_ref() {
                // Try to peek at the socket to check if it's still connected
                // This is non-intrusive and won't interfere with reader/writer threads
                let mut buf = [0u8; 1];
                match stream.peek(&mut buf) {
                    Ok(_) => {
                        // Successfully peeked, connection appears healthy
                        true
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            // No data available to peek, but connection is fine
                            true
                        }
                        std::io::ErrorKind::ConnectionAborted
                        | std::io::ErrorKind::ConnectionReset
                        | std::io::ErrorKind::UnexpectedEof 
                        | std::io::ErrorKind::NotConnected => {
                            // Connection is definitely closed
                            false
                        }
                        _ => {
                            // Other errors - assume still connected to avoid false negatives
                            true
                        }
                    }
                }
            } else {
                // Stream has been taken (during shutdown)
                false
            }
        } else {
            // Couldn't acquire lock immediately, assume connected to avoid blocking
            // This preserves the non-blocking nature of this method
            true
        }
    }
}

// VmixApiはTCP専用なので、HTTP専用のVmixApiClientトレイトは実装しない

impl VmixTcpApiClient for VmixApi {
    fn send_command(&self, command: crate::commands::SendCommand) -> Result<()> {
        self.send_command(command)
    }

    fn try_receive_command(&self, timeout: Duration) -> Result<RecvCommand> {
        self.try_receive_command(timeout)
    }

    fn is_connected(&self) -> bool {
        self.is_connected()
    }

    fn get_sender(&self) -> &SyncSender<SendCommand> {
        &self.sender
    }

    fn disconnect(&self) -> Result<()> {
        self.disconnect()
    }
}

use std::{io::Write, net::{SocketAddr, TcpStream}, sync::{mpsc::{Receiver, SyncSender}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread::JoinHandle};
use anyhow::Result;
use crate::commands::{RecvCommand, SendCommand};

pub struct VmixApi {
    pub sender: SyncSender<SendCommand>,
    pub receiver: Receiver<RecvCommand>,
    shutdown_signal: Arc<AtomicBool>,
    reader_handle: Option<JoinHandle<()>>,
    writer_handle: Option<JoinHandle<()>>,
}

impl Drop for VmixApi {
    fn drop(&mut self) {
        // Signal all threads to shutdown
        self.shutdown_signal.store(true, Ordering::Relaxed);
        
        // Send a final message to wake up the writer thread if it's waiting
        let _ = self.sender.try_send(SendCommand::QUIT);
        
        // Wait for threads to complete
        if let Some(handle) = self.reader_handle.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.writer_handle.take() {
            let _ = handle.join();
        }
    }
}

impl VmixApi {
    pub async fn new(
        remote: SocketAddr,
        timeout: Duration,
    ) -> Result<Self> {
        // Connect with proper error handling
        let stream = TcpStream::connect_timeout(&remote, timeout)
            .map_err(|e| anyhow::anyhow!("Failed to connect to {}: {}", remote, e))?;
        
        stream.set_read_timeout(Some(timeout))
            .map_err(|e| anyhow::anyhow!("Failed to set read timeout: {}", e))?;
        
        stream.set_write_timeout(Some(timeout))
            .map_err(|e| anyhow::anyhow!("Failed to set write timeout: {}", e))?;
    
        // Create writer stream with proper error handling
        let writer_stream = stream.try_clone()
            .map_err(|e| anyhow::anyhow!("Failed to clone stream for writer: {}", e))?;
    
        // Shared shutdown signal
        let shutdown_signal = Arc::new(AtomicBool::new(false));
        
        // Reader thread
        let (reader_sender, reader_receiver): (SyncSender<RecvCommand>, Receiver<RecvCommand>) =
            std::sync::mpsc::sync_channel(1);
        
        let reader_shutdown = shutdown_signal.clone();
        let reader_stream = stream.try_clone()
            .map_err(|e| anyhow::anyhow!("Failed to clone stream for reader: {}", e))?;
            
        let reader_handle = std::thread::spawn(move || {
            loop {
                // Check shutdown signal
                if reader_shutdown.load(Ordering::Relaxed) {
                    break;
                }
                
                // Try to read from stream with timeout handling
                match (&reader_stream).try_into() {
                    Ok(command) => {
                        if reader_sender.send(command).is_err() {
                            // Receiver dropped, exit thread
                            break;
                        }
                    }
                    Err(err) => {
                        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
                            match io_err.kind() {
                                std::io::ErrorKind::TimedOut => {
                                    // Timeout is expected, continue loop
                                    continue;
                                }
                                std::io::ErrorKind::ConnectionAborted |
                                std::io::ErrorKind::ConnectionReset |
                                std::io::ErrorKind::UnexpectedEof => {
                                    // Connection closed by remote
                                    eprintln!("Connection closed by remote: {}", err);
                                    break;
                                }
                                _ => {
                                    eprintln!("IO error in reader thread: {}", err);
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
    
        let (writer_sender, writer_receiver): (SyncSender<SendCommand>, Receiver<SendCommand>) =
            std::sync::mpsc::sync_channel(1);
    
        let writer_shutdown = shutdown_signal.clone();
        let writer_handle = std::thread::spawn(move || {
            let mut writer = writer_stream;
            
            loop {
                // Check shutdown signal
                if writer_shutdown.load(Ordering::Relaxed) {
                    break;
                }
                
                // Try to receive with timeout to allow checking shutdown signal
                match writer_receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(command) => {
                        // Check for quit command
                        if matches!(command, SendCommand::QUIT) {
                            let bytes: Vec<u8> = command.into();
                            if writer.write_all(&bytes).is_err() {
                                break;
                            }
                            // Quit command processed, exit thread
                            break;
                        }
                        
                        let bytes: Vec<u8> = command.into();
                        if writer.write_all(&bytes).is_err() {
                            eprintln!("Failed to write to stream");
                            break;
                        }
                        
                        if writer.flush().is_err() {
                            eprintln!("Failed to flush stream");
                            break;
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Timeout, continue to check shutdown signal
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
            reader_handle: Some(reader_handle),
            writer_handle: Some(writer_handle),
        })
    }
    
    /// Send a command to vMix
    pub fn send_command(&self, command: SendCommand) -> Result<()> {
        self.sender.send(command)
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))
    }
    
    /// Try to receive a command from vMix with timeout
    pub fn try_receive_command(&self, timeout: Duration) -> Result<RecvCommand> {
        self.receiver.recv_timeout(timeout)
            .map_err(|e| anyhow::anyhow!("Failed to receive command: {}", e))
    }
    
    /// Gracefully disconnect from vMix
    pub fn disconnect(&self) -> Result<()> {
        self.send_command(SendCommand::QUIT)
    }
    
    /// Check if the connection is still alive
    pub fn is_connected(&self) -> bool {
        !self.shutdown_signal.load(Ordering::Relaxed)
    }
}
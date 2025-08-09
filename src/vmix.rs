use crate::{
    commands::{InputNumber, RecvCommand, SendCommand, TallyData},
    models::Vmix,
    traits::{VmixApiClient, VmixTcpApiClient},
};
use anyhow::Result;
use async_trait::async_trait;
use std::{
    collections::HashMap,
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
        // Signal all threads to shutdown
        self.shutdown_signal.store(true, Ordering::Relaxed);
        
        // Explicitly close the original stream to force both reader and writer to exit
        if let Ok(mut stream_guard) = self.original_stream.lock() {
            if let Some(stream) = stream_guard.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

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
        let reader_stream = stream
            .try_clone()
            .map_err(|e| anyhow::anyhow!("Failed to clone stream for reader: {}", e))?;
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
                // Check shutdown or error signals
                if reader_shutdown.load(Ordering::Relaxed) || reader_error.load(Ordering::Relaxed) {
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
                match writer_receiver.recv_timeout(Duration::from_millis(100)) {
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
        !self.shutdown_signal.load(Ordering::Relaxed) && !self.error_signal.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl VmixApiClient for VmixApi {
    async fn execute_function(
        &self,
        function: &str,
        params: &HashMap<String, String>,
    ) -> Result<()> {
        // Build the function string with parameters
        let function_string = if params.is_empty() {
            function.to_string()
        } else {
            let params_str = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", function, params_str)
        };

        self.send_command(SendCommand::FUNCTION(function_string, None))
    }

    async fn get_xml_state(&self) -> Result<Vmix> {
        // Send XML command and wait for response
        self.send_command(SendCommand::XML)?;

        // Try to receive XML response with timeout
        let timeout = Duration::from_secs(5);
        match self.try_receive_command(timeout)? {
            RecvCommand::XML(xml_response) => {
                // Parse the XML response
                serde_xml_rs::from_str(&xml_response.body)
                    .map_err(|e| anyhow::anyhow!("Failed to parse XML response: {}", e))
            }
            other => Err(anyhow::anyhow!("Expected XML response, got: {:?}", other)),
        }
    }

    async fn get_tally_data(&self) -> Result<HashMap<InputNumber, TallyData>> {
        // Send TALLY command and wait for response
        self.send_command(SendCommand::TALLY)?;

        // Try to receive tally response with timeout
        let timeout = Duration::from_secs(5);
        match self.try_receive_command(timeout)? {
            RecvCommand::TALLY(tally_response) => Ok(tally_response.body),
            other => Err(anyhow::anyhow!("Expected TALLY response, got: {:?}", other)),
        }
    }

    async fn is_connected(&self) -> bool {
        self.is_connected()
    }

    async fn get_active_input(&self) -> Result<InputNumber> {
        let vmix_state = self.get_xml_state().await?;
        Ok(vmix_state.active.parse().unwrap_or(0))
    }

    async fn get_preview_input(&self) -> Result<InputNumber> {
        let vmix_state = self.get_xml_state().await?;
        Ok(vmix_state.preview.parse().unwrap_or(0))
    }
}

#[async_trait]
impl VmixTcpApiClient for VmixApi {
    fn try_receive_command(&self, timeout: Duration) -> Result<RecvCommand> {
        self.try_receive_command(timeout)
    }

    fn get_sender(&self) -> &SyncSender<SendCommand> {
        &self.sender
    }
}

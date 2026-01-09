use crate::commands::RecvCommand;
#[cfg(feature = "http")]
use crate::{
    commands::{InputNumber, TallyData},
    models::Vmix,
};
use anyhow::Result;
#[cfg(feature = "http")]
use async_trait::async_trait;
#[cfg(feature = "http")]
use std::collections::HashMap;

/// HTTP vMix API client trait
///
/// This trait is designed specifically for HTTP-based communication
/// with vMix instances, providing a request-response pattern.
#[cfg(feature = "http")]
#[async_trait]
pub trait VmixApiClient {
    /// Execute a vMix function with optional parameters
    ///
    /// # Arguments
    /// * `function` - The vMix function name (e.g., "Cut", "Fade", "PreviewInput")
    /// * `params` - Function parameters as key-value pairs
    ///
    /// # Example
    /// ```rust,ignore
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("Input".to_string(), "1".to_string());
    /// params.insert("Duration".to_string(), "1000".to_string());
    /// client.execute_function("Fade", &params).await?;
    /// ```
    async fn execute_function(
        &self,
        function: &str,
        params: &HashMap<String, String>,
    ) -> Result<()>;

    /// Get the complete vMix XML state
    ///
    /// Returns a structured representation of the current vMix configuration
    /// including inputs, overlays, transitions, and system state.
    async fn get_xml_state(&self) -> Result<Vmix>;

    /// Get tally data for all inputs
    ///
    /// Returns a mapping of input numbers to their tally states (OFF, PROGRAM, PREVIEW).
    /// This is useful for lighting systems and input status displays.
    async fn get_tally_data(&self) -> Result<HashMap<InputNumber, TallyData>>;

    /// Check if the client is connected and the vMix instance is responsive
    async fn is_connected(&self) -> bool;

    /// Get the currently active (program) input number
    async fn get_active_input(&self) -> Result<InputNumber>;

    /// Get the currently previewed input number
    async fn get_preview_input(&self) -> Result<InputNumber>;
}

/// TCP-specific vMix API client trait
///
/// This trait is designed specifically for TCP-based communication
/// with vMix instances. It focuses on command sending and real-time
/// event streaming without expecting specific responses to commands.
///
/// The TCP API is asynchronous by nature - you send commands and
/// receive events independently. Commands may not have immediate
/// responses, and other events (ACTS, VERSION, etc.) may arrive
/// before your command's response.
pub trait VmixTcpApiClient {
    /// Send a command to vMix (non-blocking)
    ///
    /// This method sends a command to vMix but does not wait for
    /// a specific response. Use try_receive_command to get events.
    fn send_command(&self, command: crate::commands::SendCommand) -> Result<()>;

    /// Try to receive any command/event from the TCP stream (non-blocking)
    ///
    /// This method returns immediately and provides access to real-time
    /// events from vMix such as tally updates, activator changes, XML responses, etc.
    /// The events may not correspond to your sent commands and can arrive in any order.
    fn try_receive_command(&self, timeout: std::time::Duration) -> Result<RecvCommand>;

    /// Check if the TCP connection is still alive
    fn is_connected(&self) -> bool;

    /// Get a sender for sending commands to the TCP connection
    ///
    /// This provides access to the underlying command sender for advanced
    /// use cases that need direct control over the TCP communication.
    fn get_sender(&self) -> &std::sync::mpsc::SyncSender<crate::commands::SendCommand>;

    /// Gracefully disconnect from vMix
    fn disconnect(&self) -> Result<()>;
}

/// Factory trait for creating vMix API clients
///
/// This trait allows for easy creation of different client types
/// with consistent configuration parameters. TCP and HTTP clients
/// are now completely separate with their own specialized traits.
#[cfg(feature = "http")]
pub trait VmixClientFactory {
    type TcpClient: VmixTcpApiClient + Send + Sync;
    type HttpClient: VmixApiClient + Send + Sync;

    /// Create a new TCP client
    fn create_tcp_client(
        addr: std::net::SocketAddr,
        timeout: std::time::Duration,
    ) -> impl std::future::Future<Output = Result<Self::TcpClient>> + Send;

    /// Create a new HTTP client
    fn create_http_client(
        addr: std::net::SocketAddr,
        timeout: std::time::Duration,
    ) -> Self::HttpClient;

    /// Create HTTP client with separate host and port
    fn create_http_client_with_host_port(
        host: &str,
        port: u16,
        timeout: std::time::Duration,
    ) -> Self::HttpClient;
}

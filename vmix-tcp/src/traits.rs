use crate::commands::RecvCommand;
use anyhow::Result;

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

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use vmix_core::Vmix;
use vmix_tcp::{InputNumber, TallyData};

/// HTTP vMix API client trait
///
/// This trait is designed specifically for HTTP-based communication
/// with vMix instances, providing a request-response pattern.
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

use crate::traits::VmixApiClient;
use anyhow::Result;
use async_trait::async_trait;
use shiguredo_http11::{Request, ResponseDecoder};
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use urlencoding::encode;
use vmix_core::Vmix;
use vmix_tcp::{InputNumber, TallyData};

#[derive(Debug, Clone)]
pub struct HttpVmixClient {
    host: String,
    port: u16,
    request_timeout: Duration,
}

impl HttpVmixClient {
    pub fn new(addr: SocketAddr, request_timeout: Duration) -> Self {
        Self {
            host: addr.ip().to_string(),
            port: addr.port(),
            request_timeout,
        }
    }

    pub fn new_with_host_port(host: &str, port: u16, request_timeout: Duration) -> Self {
        Self {
            host: host.to_string(),
            port,
            request_timeout,
        }
    }

    async fn send_request(&self, path: &str, query_params: &HashMap<String, String>) -> Result<Vec<u8>> {
        // Build query string
        let mut query_parts = Vec::new();
        for (key, value) in query_params {
            query_parts.push(format!("{}={}", encode(key), encode(value)));
        }
        let query_string = if query_parts.is_empty() {
            String::new()
        } else {
            format!("?{}", query_parts.join("&"))
        };

        let uri = format!("{}{}", path, query_string);

        // Create HTTP request
        let request = Request::new("GET", &uri)
            .header("Host", &format!("{}:{}", self.host, self.port))
            .header("Connection", "close");

        // Encode request
        let request_bytes = request.encode();

        // Connect to server
        let addr = format!("{}:{}", self.host, self.port);
        let mut stream = timeout(self.request_timeout, TcpStream::connect(&addr)).await??;

        // Send request
        stream.write_all(&request_bytes).await?;
        stream.flush().await?;

        // Read response
        let mut decoder = ResponseDecoder::new();
        let mut temp_buffer = [0u8; 8192];
        let mut response_opt = None;

        // Read until we get a complete response
        loop {
            let read_timeout = timeout(self.request_timeout, stream.read(&mut temp_buffer)).await?;
            match read_timeout {
                Ok(0) => break, // EOF
                Ok(n) => {
                    decoder.feed(&temp_buffer[..n])?;
                    
                    // Try to decode response
                    if let Some(response) = decoder.decode()? {
                        response_opt = Some(response);
                        break;
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Failed to read response: {}", e)),
            }
        }

        let response = response_opt.ok_or_else(|| anyhow::anyhow!("Failed to decode response"))?;

        // Check status code
        if !(200..300).contains(&response.status_code) {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status_code
            ));
        }

        // Read body
        // The decoder has already consumed the headers, so we need to read the body separately
        // Check if there's a body based on Content-Length or Transfer-Encoding
        let mut body = Vec::new();
        
        // Try to get Content-Length from headers
        let content_length = response
            .headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.parse::<usize>().ok());

        if let Some(len) = content_length {
            // Read exactly len bytes
            let mut remaining = len;
            while remaining > 0 {
                let to_read = remaining.min(temp_buffer.len());
                let read_timeout = timeout(self.request_timeout, stream.read(&mut temp_buffer[..to_read])).await?;
                match read_timeout {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        body.extend_from_slice(&temp_buffer[..n]);
                        remaining -= n;
                    }
                    Err(e) => return Err(anyhow::anyhow!("Failed to read body: {}", e)),
                }
            }
        } else {
            // No Content-Length, read until EOF (Connection: close)
            loop {
                let read_timeout = timeout(self.request_timeout, stream.read(&mut temp_buffer)).await?;
                match read_timeout {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        body.extend_from_slice(&temp_buffer[..n]);
                    }
                    Err(e) => return Err(anyhow::anyhow!("Failed to read body: {}", e)),
                }
            }
        }

        Ok(body)
    }

    pub async fn execute_function(
        &self,
        function: &str,
        params: &HashMap<String, String>,
    ) -> Result<()> {
        let mut query_params = HashMap::new();
        query_params.insert("Function".to_string(), function.to_string());
        query_params.extend(params.clone());

        self.send_request("/api", &query_params).await?;
        Ok(())
    }

    pub async fn get_xml_state(&self) -> Result<Vmix> {
        let body = self.send_request("/api", &HashMap::new()).await?;
        let xml_text = String::from_utf8(body)?;
        let vmix_data: Vmix = vmix_core::from_str(&xml_text)?;
        Ok(vmix_data)
    }

    pub async fn get_tally_data(&self) -> Result<HashMap<InputNumber, TallyData>> {
        // HTTP API doesn't have direct TALLY command, so we need to derive it from XML state
        // This simulates the TCP TALLY response by analyzing the XML state
        let vmix_state = self.get_xml_state().await?;
        let mut tally_map = HashMap::new();

        // Parse active and preview inputs to determine tally states
        let active_input: InputNumber = vmix_state.active.parse().unwrap_or(0);
        let preview_input: InputNumber = vmix_state.preview.parse().unwrap_or(0);

        // Populate tally data for all inputs (up to 1000 as per vMix spec)
        for input in &vmix_state.inputs.input {
            let input_number: InputNumber = input.number.parse().unwrap_or(0);

            let tally_state = if input_number == active_input {
                TallyData::PROGRAM
            } else if input_number == preview_input {
                TallyData::PREVIEW
            } else {
                TallyData::OFF
            };

            tally_map.insert(input_number, tally_state);
        }

        Ok(tally_map)
    }

    pub async fn is_connected(&self) -> bool {
        match self.send_request("/api", &HashMap::new()).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn get_active_input(&self) -> Result<InputNumber> {
        let vmix_data = self.get_xml_state().await?;
        Ok(vmix_data.active.parse().unwrap_or(0))
    }

    pub async fn get_preview_input(&self) -> Result<InputNumber> {
        let vmix_data = self.get_xml_state().await?;
        Ok(vmix_data.preview.parse().unwrap_or(0))
    }

    pub fn get_base_url(&self) -> String {
        format!("http://{}:{}/api", self.host, self.port)
    }
}

// Helper function for common vMix functions
impl HttpVmixClient {
    pub async fn cut(&self) -> Result<()> {
        self.execute_function("Cut", &HashMap::new()).await
    }

    pub async fn fade(&self, duration_ms: Option<u32>) -> Result<()> {
        let mut params = HashMap::new();
        if let Some(duration) = duration_ms {
            params.insert("Duration".to_string(), duration.to_string());
        }
        self.execute_function("Fade", &params).await
    }

    pub async fn preview_input(&self, input: InputNumber) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("Input".to_string(), input.to_string());
        self.execute_function("PreviewInput", &params).await
    }

    pub async fn active_input(&self, input: InputNumber) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("Input".to_string(), input.to_string());
        self.execute_function("ActiveInput", &params).await
    }

    pub async fn set_text(
        &self,
        input: InputNumber,
        selected_name: &str,
        value: &str,
    ) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("Input".to_string(), input.to_string());
        params.insert("SelectedName".to_string(), selected_name.to_string());
        params.insert("Value".to_string(), value.to_string());
        self.execute_function("SetText", &params).await
    }

    pub async fn start_recording(&self) -> Result<()> {
        self.execute_function("StartRecording", &HashMap::new())
            .await
    }

    pub async fn stop_recording(&self) -> Result<()> {
        self.execute_function("StopRecording", &HashMap::new())
            .await
    }

    pub async fn start_streaming(&self) -> Result<()> {
        self.execute_function("StartStreaming", &HashMap::new())
            .await
    }

    pub async fn stop_streaming(&self) -> Result<()> {
        self.execute_function("StopStreaming", &HashMap::new())
            .await
    }
}

// HttpVmixClientはマルチスレッド環境で安全に使用できる
unsafe impl Send for HttpVmixClient {}
unsafe impl Sync for HttpVmixClient {}

#[async_trait]
impl VmixApiClient for HttpVmixClient {
    async fn execute_function(
        &self,
        function: &str,
        params: &HashMap<String, String>,
    ) -> Result<()> {
        self.execute_function(function, params).await
    }

    async fn get_xml_state(&self) -> Result<Vmix> {
        self.get_xml_state().await
    }

    async fn get_tally_data(&self) -> Result<HashMap<InputNumber, TallyData>> {
        self.get_tally_data().await
    }

    async fn is_connected(&self) -> bool {
        self.is_connected().await
    }

    async fn get_active_input(&self) -> Result<InputNumber> {
        self.get_active_input().await
    }

    async fn get_preview_input(&self) -> Result<InputNumber> {
        self.get_preview_input().await
    }
}

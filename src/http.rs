use crate::{
    commands::{InputNumber, TallyData},
    models::Vmix,
    traits::VmixApiClient,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_xml_rs as xml;
use std::{collections::HashMap, net::SocketAddr, time::Duration};

#[derive(Debug, Clone)]
pub struct HttpVmixClient {
    base_url: String,
    client: Client,
}

impl HttpVmixClient {
    pub fn new(addr: SocketAddr, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: format!("http://{}/api", addr),
            client,
        }
    }

    pub fn new_with_host_port(host: &str, port: u16, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: format!("http://{}:{}/api", host, port),
            client,
        }
    }

    pub async fn execute_function(
        &self,
        function: &str,
        params: &HashMap<String, String>,
    ) -> Result<()> {
        let mut url = reqwest::Url::parse(&self.base_url)?;

        // Add Function parameter
        url.query_pairs_mut().append_pair("Function", function);

        // Add all parameters from HashMap
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        let response = self.client.get(url.as_str()).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ))
        }
    }

    pub async fn get_xml_state(&self) -> Result<Vmix> {
        let response = self.client.get(&self.base_url).send().await?;

        let xml_text = response.text().await?;
        let vmix_data: Vmix = xml::from_str(&xml_text)?;
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
        match self.client.get(&self.base_url).send().await {
            Ok(response) => response.status().is_success(),
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

    pub fn get_base_url(&self) -> &str {
        &self.base_url
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

#[async_trait(?Send)]
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

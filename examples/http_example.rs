use anyhow::Result;
use std::{collections::HashMap, time::Duration};
use vmix_rs::http::HttpVmixClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Create HTTP client
    let client = HttpVmixClient::new_with_host_port("192.168.1.6", 8088, Duration::from_secs(10));

    println!("Testing vMix HTTP API Client");
    println!("=============================");

    // Test connection
    println!("Testing connection...");
    if client.is_connected().await {
        println!("✅ Successfully connected to vMix");
    } else {
        println!("❌ Failed to connect to vMix");
        return Ok(());
    }

    // Get XML state
    println!("\nGetting vMix state...");
    match client.get_xml_state().await {
        Ok(state) => {
            println!("✅ vMix Version: {}", state.version);
            println!("✅ vMix Edition: {}", state.edition);
            println!("✅ Active Input: {}", state.active);
            println!("✅ Preview Input: {}", state.preview);
            println!("✅ Total Inputs: {}", state.inputs.input.len());

            // Test VideoList item parsing
            println!("\nTesting VideoList item parsing...");
            let video_list_inputs: Vec<_> = state
                .inputs
                .input
                .iter()
                .filter(|input| input.input_type == "VideoList")
                .collect();

            if video_list_inputs.is_empty() {
                println!("⚠️  No VideoList inputs found in vMix");
            } else {
                println!("✅ Found {} VideoList input(s)", video_list_inputs.len());

                for input in video_list_inputs.iter().take(2) {
                    println!("   VideoList '{}' (Input {})", input.title, input.number);
                    if let Some(list) = &input.list {
                        println!("     List has {} items", list.item.len());
                        for (i, item) in list.item.iter().take(3).enumerate() {
                            println!(
                                "       Item {}: enabled={:?}, selected={:?}, text={:?}",
                                i, item.enabled, item.selected, item.text
                            );
                        }
                    } else {
                        println!("     No list data found");
                    }
                }
            }
        }
        Err(e) => println!("❌ Failed to get vMix state: {}", e),
    }

    // Get tally data
    println!("\nGetting tally data...");
    match client.get_tally_data().await {
        Ok(tally_data) => {
            println!("✅ Tally data retrieved for {} inputs", tally_data.len());

            // Show first few tally states
            for (input_num, tally_state) in tally_data.iter().take(5) {
                println!("   Input {}: {:?}", input_num, tally_state);
            }
        }
        Err(e) => println!("❌ Failed to get tally data: {}", e),
    }

    // Test function execution (safe commands only)
    println!("\nTesting function execution...");

    // Example: Fade transition
    let mut params = HashMap::new();
    params.insert("Duration".to_string(), "1000".to_string());

    match client.execute_function("Fade", &params).await {
        Ok(_) => println!("✅ Fade command executed successfully"),
        Err(e) => println!("⚠️  Fade command failed (this might be expected): {}", e),
    }

    // Example: Cut transition (no parameters)
    match client.execute_function("Cut", &HashMap::new()).await {
        Ok(_) => println!("✅ Cut command executed successfully"),
        Err(e) => println!("⚠️  Cut command failed (this might be expected): {}", e),
    }

    // Test convenience methods
    println!("\nTesting convenience methods...");

    match client.get_active_input().await {
        Ok(active) => println!("✅ Active input: {}", active),
        Err(e) => println!("❌ Failed to get active input: {}", e),
    }

    match client.get_preview_input().await {
        Ok(preview) => println!("✅ Preview input: {}", preview),
        Err(e) => println!("❌ Failed to get preview input: {}", e),
    }

    println!("\nHTTP Client test completed!");
    Ok(())
}

use anyhow::Result;
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use vmix_rs::{http::HttpVmixClient, traits::VmixApiClient, vmix::VmixApi};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "192.168.1.6:8099".parse()?; // TCP port

    println!("vMix TCP vs HTTP API Comparison");
    println!("================================");

    // Create both clients
    println!("Creating clients...");
    let tcp_client = VmixApi::new(addr, Duration::from_secs(5)).await?;
    let http_client =
        HttpVmixClient::new_with_host_port("192.168.1.6", 8088, Duration::from_secs(5));

    println!("✅ Both clients created successfully");

    // Test connection status
    println!("\n1. Connection Status:");
    println!("   TCP Connected:  {}", tcp_client.is_connected());
    println!("   HTTP Connected: {}", http_client.is_connected().await);

    // Compare XML state retrieval performance
    println!("\n2. XML State Retrieval:");

    let start = std::time::Instant::now();
    match tcp_client.get_xml_state().await {
        Ok(state) => {
            let duration = start.elapsed();
            println!(
                "   TCP:  ✅ Retrieved in {:?} - {} inputs",
                duration,
                state.inputs.input.len()
            );
        }
        Err(e) => println!("   TCP:  ❌ Failed: {}", e),
    }

    let start = std::time::Instant::now();
    match http_client.get_xml_state().await {
        Ok(state) => {
            let duration = start.elapsed();
            println!(
                "   HTTP: ✅ Retrieved in {:?} - {} inputs",
                duration,
                state.inputs.input.len()
            );
        }
        Err(e) => println!("   HTTP: ❌ Failed: {}", e),
    }

    // Compare tally data retrieval
    println!("\n3. Tally Data Retrieval:");

    let start = std::time::Instant::now();
    match tcp_client.get_tally_data().await {
        Ok(tally_data) => {
            let duration = start.elapsed();
            println!(
                "   TCP:  ✅ Retrieved in {:?} - {} inputs",
                duration,
                tally_data.len()
            );
        }
        Err(e) => println!("   TCP:  ❌ Failed: {}", e),
    }

    let start = std::time::Instant::now();
    match http_client.get_tally_data().await {
        Ok(tally_data) => {
            let duration = start.elapsed();
            println!(
                "   HTTP: ✅ Retrieved in {:?} - {} inputs",
                duration,
                tally_data.len()
            );
        }
        Err(e) => println!("   HTTP: ❌ Failed: {}", e),
    }

    // Test function execution
    println!("\n4. Function Execution (Cut):");
    let params = HashMap::new();

    let start = std::time::Instant::now();
    match tcp_client.execute_function("Cut", &params).await {
        Ok(_) => {
            let duration = start.elapsed();
            println!("   TCP:  ✅ Executed in {:?}", duration);
        }
        Err(e) => println!("   TCP:  ❌ Failed: {}", e),
    }

    // Wait a bit before HTTP request
    tokio::time::sleep(Duration::from_millis(100)).await;

    let start = std::time::Instant::now();
    match http_client.execute_function("Cut", &params).await {
        Ok(_) => {
            let duration = start.elapsed();
            println!("   HTTP: ✅ Executed in {:?}", duration);
        }
        Err(e) => println!("   HTTP: ❌ Failed: {}", e),
    }

    // Demonstrate TCP-specific real-time capabilities
    println!("\n5. TCP Real-time Event Handling:");
    println!("   Attempting to receive real-time events (5 second timeout)...");

    match tcp_client.try_receive_command(Duration::from_secs(5)) {
        Ok(event) => println!("   ✅ Received event: {:?}", event),
        Err(e) => println!("   ⏰ No events within timeout (this is normal): {}", e),
    }

    println!("   Note: HTTP API does not support real-time events");

    // Summary
    println!("\n6. Summary:");
    println!("   TCP API:");
    println!("   + Real-time event streaming");
    println!("   + Lower latency for frequent operations");
    println!("   + Persistent connection");
    println!("   - More complex connection management");
    println!("   - Requires connection handling");

    println!("\n   HTTP API:");
    println!("   + Simple request/response model");
    println!("   + Built-in timeout handling");
    println!("   + No connection state management");
    println!("   - No real-time events");
    println!("   - Higher latency per request");

    println!("\n✅ Comparison completed successfully!");

    // Gracefully disconnect TCP client
    let _ = tcp_client.disconnect();

    Ok(())
}

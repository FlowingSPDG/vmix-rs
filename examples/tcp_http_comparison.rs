use anyhow::Result;
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use vmix_rs::{
    commands::{SUBSCRIBECommand, SendCommand},
    http::HttpVmixClient,
    vmix::VmixApi,
};

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

    // Compare command sending performance (TCP) vs request-response (HTTP)
    println!("\n2. Command Execution Comparison:");

    // TCP: Send command without waiting for response
    println!("   TCP Command Sending (async, no response wait):");
    let start = std::time::Instant::now();
    match tcp_client.send_command(SendCommand::XML) {
        Ok(_) => {
            let duration = start.elapsed();
            println!("      ✅ XML command sent in {:?}", duration);
        }
        Err(e) => println!("      ❌ Failed to send XML command: {}", e),
    }

    // TCP: Try to receive any event (may or may not be related to our command)
    match tcp_client.try_receive_command(Duration::from_secs(1)) {
        Ok(event) => println!("      ✅ Received event: {:?}", event),
        Err(e) => println!("      ⏰ No events within timeout: {}", e),
    }

    // HTTP: Request with guaranteed response
    println!("\n   HTTP Request-Response (synchronous):");
    let start = std::time::Instant::now();
    match http_client.get_xml_state().await {
        Ok(state) => {
            let duration = start.elapsed();
            println!(
                "      ✅ Retrieved XML state in {:?} - {} inputs",
                duration,
                state.inputs.input.len()
            );
        }
        Err(e) => println!("      ❌ Failed: {}", e),
    }

    // Compare function execution
    println!("\n3. Function Execution:");
    let params = HashMap::new();

    // TCP: Send function command (fire-and-forget)
    let start = std::time::Instant::now();
    match tcp_client.send_command(SendCommand::FUNCTION("Cut".to_string(), None)) {
        Ok(_) => {
            let duration = start.elapsed();
            println!("   TCP:  ✅ Cut command sent in {:?}", duration);
        }
        Err(e) => println!("   TCP:  ❌ Failed: {}", e),
    }

    // Wait a bit before HTTP request
    tokio::time::sleep(Duration::from_millis(100)).await;

    // HTTP: Execute function with response confirmation
    let start = std::time::Instant::now();
    match http_client.execute_function("Cut", &params).await {
        Ok(_) => {
            let duration = start.elapsed();
            println!("   HTTP: ✅ Cut executed and confirmed in {:?}", duration);
        }
        Err(e) => println!("   HTTP: ❌ Failed: {}", e),
    }

    // Demonstrate TCP-specific real-time capabilities
    println!("\n4. TCP Real-time Event Streaming:");
    println!("   Subscribing to TALLY updates...");

    match tcp_client.send_command(SendCommand::SUBSCRIBE(SUBSCRIBECommand::TALLY)) {
        Ok(_) => println!("   ✅ TALLY subscription sent"),
        Err(e) => println!("   ❌ Failed to subscribe: {}", e),
    }

    println!("   Listening for events (3 second timeout)...");
    for i in 1..=3 {
        match tcp_client.try_receive_command(Duration::from_secs(1)) {
            Ok(event) => println!("   Event {}: {:?}", i, event),
            Err(e) => println!("   Timeout {}: {}", i, e),
        }
    }

    println!("   Note: HTTP API does not support real-time event streaming");

    // Test HTTP tally data retrieval for comparison
    println!("\n5. HTTP Tally Data (snapshot):");
    let start = std::time::Instant::now();
    match http_client.get_tally_data().await {
        Ok(tally_data) => {
            let duration = start.elapsed();
            println!(
                "   ✅ HTTP tally snapshot retrieved in {:?} - {} inputs",
                duration,
                tally_data.len()
            );
        }
        Err(e) => println!("   ❌ HTTP tally failed: {}", e),
    }

    // Summary
    println!("\n6. Architecture Comparison:");
    println!("   TCP API:");
    println!("   + Real-time event streaming and subscriptions");
    println!("   + Fire-and-forget command sending (low latency)");
    println!("   + Persistent connection with continuous data flow");
    println!("   + Ideal for real-time applications (tally lights, live switching)");
    println!("   - Requires connection state management");
    println!("   - Event order not guaranteed to match command order");

    println!("\n   HTTP API:");
    println!("   + Simple request/response model with guaranteed responses");
    println!("   + Built-in error handling and timeout management");
    println!("   + Stateless - no connection management needed");
    println!("   + Ideal for control panels and configuration tools");
    println!("   - No real-time event capabilities");
    println!("   - Higher latency due to HTTP overhead");

    println!("\n✅ Comparison completed successfully!");

    // Gracefully disconnect TCP client
    println!("\nDisconnecting TCP client...");
    match tcp_client.disconnect() {
        Ok(_) => println!("✅ TCP client disconnected cleanly"),
        Err(e) => println!("❌ TCP disconnect error: {}", e),
    }

    Ok(())
}

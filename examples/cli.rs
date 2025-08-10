use anyhow::Result;
use std::{io::{stdin, Write}, net::SocketAddr, time::Duration};
use tokio::sync::mpsc::unbounded_channel;
use vmix_rs::{
    commands::{RecvCommand, SendCommand},
    traits::VmixTcpApiClient,
    vmix::VmixApi,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8099".parse()?;
    
    println!("Attempting to connect to vMix at {}...", addr);
    let vmix = match VmixApi::new(addr, Duration::from_secs(2)).await {
        Ok(api) => {
            println!("✅ Successfully connected to vMix!");
            api
        }
        Err(e) => {
            println!("❌ Failed to connect to vMix: {}", e);
            println!("💡 Make sure vMix is running and TCP API is enabled on port 8099");
            return Ok(());
        }
    };

    let (command_sender, mut command_receiver) = unbounded_channel();

    // Get sender before moving vmix into task
    let vmix_sender = vmix.get_sender().clone();

    // Command receiver task with proper error handling and connection monitoring
    let command_receiver_task = tokio::spawn(async move {
        loop {
            match vmix.try_receive_command(Duration::from_millis(100)) {
                Ok(received) => match received {
                    RecvCommand::TALLY(tally) => {
                        println!("📊 TALLY: {:?}", tally)
                    }
                    RecvCommand::FUNCTION(func) => {
                        println!("⚙️  FUNCTION: {:?}", func)
                    }
                    RecvCommand::ACTS(acts) => {
                        println!("🎬 ACTS: {:?}", acts)
                    }
                    RecvCommand::XML(xml) => {
                        println!("📄 XML: (length: {} chars)", xml.body.len())
                    }
                    RecvCommand::XMLTEXT(text) => {
                        println!("📝 XMLTEXT: {:?}", text)
                    }
                    RecvCommand::SUBSCRIBE(subbed) => {
                        println!("🔔 SUBSCRIBED: {:?}", subbed)
                    }
                    RecvCommand::UNSUBSCRIBE(unsubbed) => {
                        println!("🔕 UNSUBSCRIBED: {:?}", unsubbed)
                    }
                    RecvCommand::QUIT => {
                        println!("👋 QUIT received - disconnecting");
                        break;
                    }
                    RecvCommand::VERSION(version) => {
                        println!("🏷️  VERSION: {:?}", version)
                    }
                },
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("timeout") || error_msg.contains("timed out") {
                        // Timeout is expected in normal operation - just check connection
                        if !vmix.is_connected() {
                            println!("💔 Connection lost");
                            break;
                        }
                        // Connection is still alive, continue listening
                        continue;
                    } else {
                        eprintln!("❌ Error receiving command: {}", e);
                        break;
                    }
                }
            }
        }
        println!("📡 Command receiver task ended");
    });

    // Command sender task with proper error handling
    let command_sender_task = tokio::spawn(async move {
        while let Some(command) = command_receiver.recv().await {
            if let Err(e) = vmix_sender.send(command) {
                eprintln!("❌ Failed to send command: {}", e);
                break;
            }
        }
        println!("📤 Command sender task ended");
    });

    // Periodic command sender (with error handling)
    let command_sender_clone = command_sender.clone();
    let periodic_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10)); // Reduced frequency
        loop {
            interval.tick().await;
            // Send a less disruptive command
            if let Err(e) = command_sender_clone.send(SendCommand::VERSION) {
                eprintln!("❌ Failed to send periodic command: {}", e);
                break;
            }
        }
    });

    println!("\n🚀 RUNNING... 

Commands you can try:
  - 'xml'     : Get vMix state
  - 'tally'   : Get tally data  
  - 'version' : Get vMix version
  - 'cut'     : Execute cut transition
  - 'quit'    : Exit application
  - Or type any raw vMix command

Type commands and press Enter, or Ctrl+C to exit:\n");

    // Command input from stdin with proper error handling
    let input_task = tokio::task::spawn_blocking(move || {
        loop {
            print!("vmix> ");
            std::io::stdout().flush().unwrap();
            
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let trimmed = buffer.trim().to_lowercase();
                    
                    let command = match trimmed.as_str() {
                        "quit" | "exit" => {
                            println!("👋 Exiting...");
                            SendCommand::QUIT
                        },
                        "xml" => SendCommand::XML,
                        "tally" => SendCommand::TALLY, 
                        "version" => SendCommand::VERSION,
                        "cut" => SendCommand::FUNCTION("Cut".to_string(), None),
                        "fade" => SendCommand::FUNCTION("Fade".to_string(), None),
                        _ => {
                            if trimmed.is_empty() {
                                continue;
                            }
                            SendCommand::RAW(buffer)
                        }
                    };
                    
                    if let Err(e) = command_sender.send(command) {
                        eprintln!("❌ Failed to send command: {}", e);
                        break;
                    }
                    
                    if matches!(trimmed.as_str(), "quit" | "exit") {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading from stdin: {}", e);
                    break;
                }
            }
        }
        println!("⌨️  Input task ended");
    });

    // Wait for any task to complete or Ctrl+C (graceful shutdown)
    tokio::select! {
        _ = command_receiver_task => {
            println!("📡 Command receiver task completed");
        },
        _ = command_sender_task => {
            println!("📤 Command sender task completed");
        },
        _ = periodic_task => {
            println!("⏰ Periodic task completed");
        },
        _ = input_task => {
            println!("⌨️  Input task completed");
        },
        _ = tokio::signal::ctrl_c() => {
            println!("\n🛑 Received Ctrl+C, shutting down gracefully...");
            
            // Force abort all tasks to ensure quick shutdown
            println!("🔄 Application shutting down...");
            std::process::exit(0);
        },
    }

    println!("🔄 Application shutting down...");
    Ok(())
}

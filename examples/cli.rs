use anyhow::Result;
use std::{io::stdin, net::SocketAddr, time::Duration};
use tokio::sync::mpsc::unbounded_channel;
use vmix_rs::{
    commands::{RecvCommand, SendCommand},
    vmix::VmixApi,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8099".parse()?;
    let vmix = VmixApi::new(addr, Duration::from_secs(2)).await?;
    let (command_sender, mut command_receiver) = unbounded_channel();

    // Clone the sender for different tasks
    let sender_for_input = vmix.sender.clone();
    let sender_for_periodic = vmix.sender.clone();

    // Command receiver task with proper error handling and connection monitoring
    let command_receiver_task = tokio::spawn(async move {
        loop {
            match vmix.try_receive_command(Duration::from_millis(100)) {
                Ok(received) => match received {
                    RecvCommand::TALLY(tally) => {
                        println!("recv tally {:?}", tally)
                    }
                    RecvCommand::FUNCTION(func) => {
                        println!("recv func {:?}", func)
                    }
                    RecvCommand::ACTS(acts) => {
                        println!("recv acts {:?}", acts)
                    }
                    RecvCommand::XML(xml) => {
                        println!("recv xml (length: {} chars)", xml.body.len())
                    }
                    RecvCommand::XMLTEXT(text) => {
                        println!("recv text {:?}", text)
                    }
                    RecvCommand::SUBSCRIBE(subbed) => {
                        println!("recv subbed {:?}", subbed)
                    }
                    RecvCommand::UNSUBSCRIBE(unsubbed) => {
                        println!("recv unsubbed {:?}", unsubbed)
                    }
                    RecvCommand::QUIT => {
                        println!("recv quit - disconnecting");
                        break;
                    }
                    RecvCommand::VERSION(version) => {
                        println!("recv version {:?}", version)
                    }
                },
                Err(e) => {
                    if e.to_string().contains("timeout") {
                        // Timeout is expected, check if connection is still alive
                        if !vmix.is_connected() {
                            println!("Connection lost");
                            break;
                        }
                        continue;
                    } else {
                        eprintln!("Error receiving command: {}", e);
                        break;
                    }
                }
            }
        }
        println!("Command receiver task ended");
    });

    // Command sender task with proper error handling
    let command_sender_task = tokio::spawn(async move {
        while let Some(command) = command_receiver.recv().await {
            if let Err(e) = sender_for_input.send(command) {
                eprintln!("Failed to send command: {}", e);
                break;
            }
        }
        println!("Command sender task ended");
    });

    // Periodic command sender (with error handling)
    let command_sender_clone = command_sender.clone();
    let periodic_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            if let Err(e) = sender_for_periodic.send(SendCommand::FUNCTION("CUT".to_string(), None))
            {
                eprintln!("Failed to send periodic command: {}", e);
                break;
            }
        }
        println!("Periodic task ended");
    });

    println!("RUNNING... (Type commands and press Enter, or 'quit' to exit)");

    // Command input from stdin with proper error handling
    let input_task = tokio::task::spawn_blocking(move || {
        loop {
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let trimmed = buffer.trim();
                    if trimmed == "quit" {
                        println!("Exiting...");
                        if let Err(e) = command_sender_clone.send(SendCommand::QUIT) {
                            eprintln!("Failed to send quit command: {}", e);
                        }
                        break;
                    }
                    if let Err(e) = command_sender_clone.send(SendCommand::RAW(buffer)) {
                        eprintln!("Failed to send raw command: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
        println!("Input task ended");
    });

    // Wait for any task to complete (graceful shutdown)
    tokio::select! {
        _ = command_receiver_task => {},
        _ = command_sender_task => {},
        _ = periodic_task => {},
        _ = input_task => {},
    }

    println!("Application shutting down");
    Ok(())
}

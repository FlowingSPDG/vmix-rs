use anyhow::Result;
use std::{io::stdin, net::SocketAddr, time::Duration};
use tokio::sync::mpsc::unbounded_channel;
use vmix::{connect_vmix_tcp, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8099".parse()?;
    let (sender, receiver) = connect_vmix_tcp(addr, Duration::from_secs(2)).await?;
    let (command_sender, mut command_receiver) = unbounded_channel();

    tokio::spawn(async move {
        loop {
            if let Ok(received) = receiver.recv() {
                match received {
                    Command::TALLY(tally) => {
                        println!("recv tally {:?}", tally)
                    }
                    Command::FUNCTION(func) => {
                        println!("recv func {:?}", func)
                    }
                    Command::ACTS(acts) => {
                        println!("recv acts {:?}", acts)
                    }
                    Command::XML(xml) => {
                        println!("recv xml {:?}", xml.body.version)
                    }
                    Command::XMLTEXT(text) => {
                        println!("recv text {:?}", text)
                    }
                    Command::SUBSCRIBE(subbed) => {
                        println!("recv subbed {:?}", subbed)
                    }
                    Command::UNSUBSCRIBE(unsubbed) => {
                        println!("recv unsubbed {:?}", unsubbed)
                    }
                    Command::QUIT => {
                        println!("recv quit")
                    }
                    Command::VERSION(version) => {
                        println!("recv version {:?}", version)
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let command = command_receiver.recv().await.unwrap();
            // let command = "FUNCTION CUT\r\n".to_string();
            sender.send(command).unwrap();
        }
    });

    println!("RUNNING...");

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        command_sender.send(buffer).unwrap();
    }
}

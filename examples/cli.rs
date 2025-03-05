use anyhow::Result;
use std::{io::stdin, net::SocketAddr, time::Duration};
use tokio::sync::mpsc::unbounded_channel;
use vmix::{commands::{RecvCommand, SendCommand}, vmix::VmixApi};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8099".parse()?;
    let vmix = VmixApi::new(addr, Duration::from_secs(2)).await?;
    let (command_sender, mut command_receiver) = unbounded_channel();

    let receiver = vmix.receiver;
    let sender = vmix.sender;

    // コマンド受信
    tokio::spawn(async move {
        loop {
            if let Ok(received) = receiver.recv() {
                match received {
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
                        println!("recv xml {:?}", xml.body.version)
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
                        println!("recv quit")
                    }
                    RecvCommand::VERSION(version) => {
                        println!("recv version {:?}", version)
                    }
                }
            }
        }
    });

    // コマンド送信
    tokio::spawn(async move {
        loop {
            let command = command_receiver.recv().await.unwrap();
            // let command = "FUNCTION CUT\r\n".to_string();
            sender.send(command).unwrap();
        }
    });

    let command_sender_clone = command_sender.clone();
    tokio::spawn(async move {
        loop {
            command_sender.send(SendCommand::FUNCTION("CUT".to_string(), None)).unwrap();
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });

    println!("RUNNING...");

    // コマンド送信(標準入力からの読み込み)
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        command_sender_clone.send(SendCommand::RAW(buffer)).unwrap();
    }
}

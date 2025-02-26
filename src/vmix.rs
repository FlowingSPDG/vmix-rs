use std::{io::Write, net::{SocketAddr, TcpStream}, sync::mpsc::{Receiver, SyncSender}, time::Duration};
use anyhow::Result;
use crate::commands::{RecvCommand, SendCommand};

pub struct VmixApi {
    pub sender: SyncSender<SendCommand>,
    pub receiver: Receiver<RecvCommand>,
}

impl VmixApi {
    pub async fn new(
        remote: SocketAddr,
        timeout: Duration,
    ) -> Result<Self> {
        let stream = TcpStream::connect_timeout(&remote, timeout).expect("Could not connect.");
        stream.set_read_timeout(None).unwrap();
    
        // writer stream
        let mut writer = stream.try_clone().unwrap();
    
        // reader thread
        let (reader_sender, reader_receiver): (SyncSender<RecvCommand>, Receiver<RecvCommand>) =
            std::sync::mpsc::sync_channel(1);
        tokio::spawn(async move {
            // reader stream
            let reader = &stream.try_clone().unwrap();
            loop {
                let read_result: Result<RecvCommand, _> = reader.try_into();
                if let Err(err) = read_result {
                    if err.is::<std::io::ErrorKind>() {
                        // IO ERROR (e.g. timeout)
                        println!("Connection is closed");
                        break;
                    }
                    println!("Failed to parse incoming packet: {}", err);
                    continue;
                }
                reader_sender.send(read_result.unwrap()).unwrap();
            }
        });
    
        let (writer_sender, writer_receiver): (SyncSender<SendCommand>, Receiver<SendCommand>) =
            std::sync::mpsc::sync_channel(1);
    
        tokio::spawn(async move {
            loop {
                let received = writer_receiver.recv();
                if let Ok(received) = received {
                    let bytes: Vec<u8> = received.into();
                    writer.write_all(&bytes).expect("failed to write");
                }
            }
        });
        Ok(Self { sender: writer_sender, receiver: reader_receiver })
    }
    
}

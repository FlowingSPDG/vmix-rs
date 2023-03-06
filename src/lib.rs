use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::mpsc::{Receiver, SyncSender},
    time::Duration,
};

// TODO: add "Request" struct

pub use crate::commands::Command;
use anyhow::Result;

pub mod acts;
pub mod commands;
pub mod models;

pub async fn connect_vmix_tcp(
    remote: SocketAddr,
    timeout: Duration,
) -> Result<(SyncSender<String>, Receiver<Command>)> {
    let stream = TcpStream::connect_timeout(&remote, timeout).expect("Could not connect.");
    stream.set_read_timeout(None).unwrap();

    // writer stream
    let mut writer = stream.try_clone().unwrap();

    // reader thread
    let (reader_sender, reader_receiver): (SyncSender<Command>, Receiver<Command>) =
        std::sync::mpsc::sync_channel(1);
    tokio::spawn(async move {
        // reader stream
        let reader = &stream.try_clone().unwrap();
        loop {
            let read_result: Result<Command, _> = reader.try_into();
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

    let (writer_sender, writer_receiver): (SyncSender<String>, Receiver<String>) =
        std::sync::mpsc::sync_channel(1);

    tokio::spawn(async move {
        loop {
            let received = writer_receiver.recv();
            if let Ok(received) = received {
                writer
                    .write_all(received.as_bytes())
                    .expect("failed to write");
            }
        }
    });
    Ok((writer_sender, reader_receiver))
}

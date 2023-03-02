use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vmix {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "edition")]
    pub edition: String,

    #[serde(rename = "preset")]
    pub preset: String,

    #[serde(rename = "inputs")]
    pub inputs: Inputs,

    #[serde(rename = "overlays")]
    pub overlays: Overlays,

    #[serde(rename = "preview")]
    pub preview: String,

    #[serde(rename = "active")]
    pub active: String,

    #[serde(rename = "fadeToBlack")]
    pub fade_to_black: Boolean,

    #[serde(rename = "transitions")]
    pub transitions: Transitions,

    #[serde(rename = "recording")]
    pub recording: Boolean,

    #[serde(rename = "external")]
    pub external: Boolean,

    #[serde(rename = "streaming")]
    pub streaming: Boolean,

    #[serde(rename = "playList")]
    pub play_list: Boolean,

    #[serde(rename = "multiCorder")]
    pub multi_corder: Boolean,

    #[serde(rename = "fullscreen")]
    pub fullscreen: Boolean,

    #[serde(rename = "audio")]
    pub audio: Audio,

    #[serde(rename = "dynamic")]
    pub dynamic: Dynamic,
}

#[derive(Serialize, Deserialize)]
pub struct Audio {
    #[serde(rename = "master")]
    master: Master,
}

#[derive(Serialize, Deserialize)]
pub struct Master {
    #[serde(rename = "volume")]
    volume: String,

    #[serde(rename = "muted")]
    muted: Boolean,

    #[serde(rename = "meterF1")]
    meter_f1: String,

    #[serde(rename = "meterF2")]
    meter_f2: String,

    #[serde(rename = "headphonesVolume")]
    headphones_volume: String,
}

#[derive(Serialize, Deserialize)]
pub struct Dynamic {
    #[serde(rename = "input1")]
    input1: String,

    #[serde(rename = "input2")]
    input2: String,

    #[serde(rename = "input3")]
    input3: String,

    #[serde(rename = "input4")]
    input4: String,

    #[serde(rename = "value1")]
    value1: String,

    #[serde(rename = "value2")]
    value2: String,

    #[serde(rename = "value3")]
    value3: String,

    #[serde(rename = "value4")]
    value4: String,
}

#[derive(Serialize, Deserialize)]
pub struct Inputs {
    #[serde(rename = "input")]
    pub input: Vec<Input>,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    // #[serde(rename = "overlay")]
    // overlay: Option<OverlayUnion>,
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "number")]
    pub number: String,

    #[serde(rename = "type")]
    pub input_type: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "shortTitle")]
    pub short_title: String,

    #[serde(rename = "state")]
    pub state: State,

    #[serde(rename = "position")]
    pub position: String,

    #[serde(rename = "duration")]
    pub duration: String,

    #[serde(rename = "loop")]
    pub input_loop: Boolean,

    //#[serde(rename = "text")]
    //text: String,
    #[serde(rename = "muted")]
    pub muted: Option<Boolean>,

    #[serde(rename = "volume")]
    pub volume: Option<String>,

    #[serde(rename = "balance")]
    pub balance: Option<String>,

    #[serde(rename = "solo")]
    pub solo: Option<Boolean>,

    #[serde(rename = "audiobusses")]
    pub audiobusses: Option<Audiobusses>,

    #[serde(rename = "meterF1")]
    pub meter_f1: Option<String>,

    #[serde(rename = "meterF2")]
    pub meter_f2: Option<String>,

    #[serde(rename = "gainDb")]
    pub gain_db: Option<String>,

    #[serde(rename = "list")]
    pub list: Option<List>,

    #[serde(rename = "selectedIndex")]
    pub selected_index: Option<String>,

    // #[serde(rename = "text")]
    // input_text: Option<Vec<Image>>,
    #[serde(rename = "image")]
    pub image: Option<Image>,

    #[serde(rename = "replay")]
    pub replay: Option<Replay>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "index")]
    index: String,

    #[serde(rename = "name")]
    name: String,
    // #[serde(rename = "_text")]
    // text: String,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    // #[serde(rename = "item")]
    // item: Vec<ItemElement>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemClass {
    #[serde(rename = "_selected")]
    selected: String,
    // #[serde(rename = "_text")]
    // text: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleOverlay {
    #[serde(rename = "_index")]
    index: String,

    #[serde(rename = "_key")]
    key: String,

    #[serde(rename = "position")]
    position: Option<Position>,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "panX")]
    pan_x: f64,

    #[serde(rename = "zoomX")]
    zoom_x: f64,

    #[serde(rename = "zoomY")]
    zoom_y: f64,

    #[serde(rename = "panY")]
    pan_y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyOverlay {
    #[serde(rename = "_index")]
    index: String,

    #[serde(rename = "_key")]
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Replay {
    #[serde(rename = "timecode")]
    timecode: String,

    #[serde(rename = "timecodeA")]
    timecode_a: String,

    #[serde(rename = "timecodeB")]
    timecode_b: String,

    #[serde(rename = "live")]
    live: Boolean,

    #[serde(rename = "recording")]
    recording: Boolean,

    #[serde(rename = "channelMode")]
    channel_mode: String,

    #[serde(rename = "events")]
    events: String,

    #[serde(rename = "eventsA")]
    events_a: String,

    #[serde(rename = "eventsB")]
    events_b: String,

    #[serde(rename = "cameraA")]
    camera_a: String,

    #[serde(rename = "cameraB")]
    camera_b: String,

    #[serde(rename = "speed")]
    speed: String,

    #[serde(rename = "speedA")]
    speed_a: String,

    #[serde(rename = "speedB")]
    speed_b: String,
}

#[derive(Serialize, Deserialize)]
pub struct Overlays {
    #[serde(rename = "overlay")]
    overlay: Vec<OverlaysOverlay>,
}

#[derive(Serialize, Deserialize)]
pub struct OverlaysOverlay {
    #[serde(rename = "number")]
    number: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transitions {
    #[serde(rename = "transition")]
    transition: Vec<Transition>,
}

#[derive(Serialize, Deserialize)]
pub struct Transition {
    #[serde(rename = "number")]
    number: String,

    #[serde(rename = "effect")]
    effect: String,

    #[serde(rename = "duration")]
    duration: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemElement {
    ItemClass(ItemClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OverlayUnion {
    FluffyOverlay(FluffyOverlay),

    PurpleOverlayArray(Vec<PurpleOverlay>),
}

#[derive(Serialize, Deserialize)]
pub enum Boolean {
    #[serde(rename = "False")]
    False,

    #[serde(rename = "True")]
    True,
}

#[derive(Serialize, Deserialize)]
pub enum Audiobusses {
    #[serde(rename = "M")]
    M,
}

#[derive(Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Paused")]
    Paused,

    #[serde(rename = "Running")]
    Running,
}

// TCP API
use crate::Status::Length;
use anyhow::Result;

#[derive(Debug)]
pub enum Command {
    TALLY,
    FUNCTION,
    ACTS,
    XML,
    XMLTEXT,
    SUBSCRIBE,
    UNSUBSCRIBE,
    QUIT,
    VERSION,
}
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "TALLY" => Ok(Self::TALLY),
            "FUNCTION" => Ok(Self::FUNCTION),
            "ACTS" => Ok(Self::ACTS),
            "XML" => Ok(Self::XML),
            "XMLTEXT" => Ok(Self::XMLTEXT),
            "SUBSCRIBE" => Ok(Self::SUBSCRIBE),
            "UNSUBSCRIBE" => Ok(Self::UNSUBSCRIBE),
            "QUIT" => Ok(Self::QUIT),
            "VERSION" => Ok(Self::VERSION),
            _ => Err(anyhow::anyhow!("No matching command found")),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    OK,             // "OK"
    ER,             // "ER"
    Length(u64),    //  Length of body
    Detail(String), // detail data
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "OK" => Self::OK,
            "ER" => Self::ER,
            _ => {
                if let Ok(length) = value.parse::<u64>() {
                    return Self::Length(length);
                };
                return Self::Detail(value.to_string());
            }
        }
    }
}

// TODO: add "Request" struct

#[derive(Debug)]
pub struct Response {
    command: Command,
    status: Status,
    body: Option<String>,
    data: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            command: Command::TALLY,
            status: Status::OK,
            body: None,
            data: None,
        }
    }
}

impl TryFrom<BufReader<&TcpStream>> for Response {
    type Error = anyhow::Error;

    fn try_from(mut stream: BufReader<&TcpStream>) -> std::result::Result<Self, Self::Error> {
        // read stream
        let mut value = String::new();
        stream.read_line(&mut value)?;

        // remove \r\n
        let value = value.lines().collect::<String>();

        let mut commands: Vec<&str> = vec![];
        let mut iter = value.split_whitespace();
        loop {
            if let Some(command) = iter.next() {
                commands.push(command);
            } else {
                break;
            }
        }

        if commands.len() == 0 {
            return Err(anyhow::anyhow!("Zero Command Length"));
        }
        // TODO: Handle error
        let command: Command = commands[0].try_into().unwrap();
        let status: Status = commands[1].into();
        match command {
            // Example Response: TALLY OK 0121...\r\n
            Command::TALLY => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            // Example Response: FUNCTION OK PreviewInput\r\n
            // Example Response: FUNCTION ER Error message\r\n
            Command::FUNCTION => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            // Example Response: ACTS OK Input 1 1\r\n
            Command::ACTS => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            /*
            Example Response: XML 37\r\n
            <vmix><version>x.x.x.x</version></vmix>
            */
            Command::XML => {
                if let Length(len) = &status {
                    let mut took = stream.take(len.to_owned());
                    let mut xml = String::new();
                    took.read_to_string(&mut xml)?;

                    // remove \r\n
                    let xml = xml.lines().collect::<String>();
                    return Ok(Self {
                        command,
                        status,
                        body: None,
                        data: Some(xml),
                    });
                }
                Err(anyhow::anyhow!("Failed to read XML"))
            }
            // Example Response: XMLTEXT OK This is the title of the first input\r\n
            Command::XMLTEXT => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            // Example Response: SUBSCRIBE OK TALLY\r\n
            Command::SUBSCRIBE => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            // Example Response: UNSUBSCRIBE OK TALLY\r\n
            Command::UNSUBSCRIBE => Ok(Self {
                command,
                status,
                body: Some(commands[2].to_string()),
                data: None,
            }),
            // No response
            Command::QUIT => {
                todo!()
            }
            Command::VERSION => {
                let status: Status = commands[1].into();
                Ok(Self {
                    command,
                    status,
                    body: Some(commands[2].to_string()),
                    data: None,
                })
            }
        }
    }
}

pub async fn connect_vmix_tcp(
    remote: SocketAddr,
    timeout: Duration,
) -> Result<(SyncSender<String>, Receiver<Response>)> {
    let stream = TcpStream::connect_timeout(&remote, timeout).expect("Could not connect.");
    stream.set_read_timeout(None).unwrap();

    // reader thread
    let (reader_sender, reader_receiver): (SyncSender<Response>, Receiver<Response>) =
        std::sync::mpsc::sync_channel(1);
    let reader = stream.try_clone().unwrap();
    tokio::spawn(async move {
        loop {
            let buf_reader = BufReader::new(&reader);
            // TODO: 切断されているか確認する
            let response: Response = buf_reader.try_into().unwrap();

            reader_sender.send(response).unwrap();
        }
    });

    let (writer_sender, writer_receiver): (SyncSender<String>, Receiver<String>) =
        std::sync::mpsc::sync_channel(1);

    let mut writer = stream.try_clone().unwrap();
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

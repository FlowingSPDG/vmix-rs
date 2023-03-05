use std::collections::HashMap;
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

    // TODO: "M,A"のようなカンマ区切りのパターンも網羅する
    /*
    #[serde(rename = "audiobusses")]
    pub audiobusses: Option<Audiobusses>,
     */
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

    #[serde(rename = "A")]
    A,

    #[serde(rename = "B")]
    B,
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
use crate::TallyData::{OFF, PREVIEW, PROGRAM};
use anyhow::Result;

pub type InputNumber = u16; // 0~1000

#[derive(Debug)]
pub enum Status {
    OK,             // "OK"
    ER,             // "ER"
    Length(u64),    // Length of body
    Detail(String), // detail data
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        let value = value.as_str();
        match value {
            "OK" => Self::OK,
            "ER" => Self::ER,
            _ => {
                if let Ok(length) = value.parse::<u64>() {
                    return Length(length);
                };
                return Self::Detail(value.to_string());
            }
        }
    }
}

#[derive(Debug)]
pub struct TallyResponse {
    pub status: Status,
    pub body: HashMap<InputNumber, TallyData>,
}
#[derive(Debug)]
pub enum TallyData {
    OFF,
    PROGRAM,
    PREVIEW,
}
impl From<char> for TallyData {
    fn from(value: char) -> Self {
        match value {
            '0' => OFF,
            '1' => PROGRAM,
            '2' => PREVIEW,
            _ => OFF, // NO MATCHING PATTERN
        }
    }
}

#[derive(Debug)]
pub struct FunctionResponse {
    pub status: Status,
    pub body: Option<String>,
}

pub struct XMLResponse {
    pub status: Status,
    pub body: Vmix,
}
#[derive(Debug)]
pub struct XMLTextResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct SubscribeResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct UnsubscribeResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct VersionResponse {
    pub status: Status,
    pub version: Option<String>, // TODO: parse semver
}

#[derive(Debug)]
pub struct ActivatorsResponse {
    pub status: Status,
    pub body: ActivatorsData,
}

#[derive(Debug)]
pub enum ActivatorsData {
    Input(InputNumber),
    InputPreview(InputNumber, bool),
    InputPlaying(InputNumber, bool),
    InputVolume(InputNumber, f64),
    InputHeadphones(InputNumber, f64),
    MasterVolume(f64),
    MasterHeadphones(f64),
    BusAVolume(f64),
    BusBVolume(f64),
    InputAudio(InputNumber, bool),
    InputSolo(InputNumber, bool),
    InputBusAAudio(InputNumber, bool),
    InputBusBAudio(InputNumber, bool),
    InputMasterAudio(InputNumber, bool),
    MasterAudio(bool),
    BusAAudio(bool),
    BusBAudio(bool),
    FadeToBlack(bool),
    Recording(bool),
    Streaming(bool),
    External(bool),
    Fullscreen(bool),
}

impl TryFrom<&[String]> for ActivatorsData {
    type Error = anyhow::Error;
    fn try_from(value: &[String]) -> std::result::Result<Self, Self::Error> {
        match value[0].as_str() {
            "Input" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                Ok(ActivatorsData::Input(input_num))
            }
            "InputPreview" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreview(input_num, is_active))
            }
            "InputPlaying" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPlaying(input_num, is_active))
            }
            "InputVolume" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let volume = value[2].parse::<f64>().unwrap();
                Ok(ActivatorsData::InputVolume(input_num, volume))
            }
            "InputHeadphones" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let volume = value[2].parse::<f64>().unwrap();
                Ok(ActivatorsData::InputHeadphones(input_num, volume))
            }
            "MasterVolume" => {
                let volume = value[1].parse::<f64>().unwrap();
                Ok(ActivatorsData::MasterVolume(volume))
            }
            "MasterHeadphones" => {
                let volume = value[1].parse::<f64>().unwrap();
                Ok(ActivatorsData::MasterHeadphones(volume))
            }
            "BusAVolume" => {
                let volume = value[1].parse::<f64>().unwrap();
                Ok(ActivatorsData::BusAVolume(volume))
            }
            "BusBVolume" => {
                let volume = value[1].parse::<f64>().unwrap();
                Ok(ActivatorsData::BusBVolume(volume))
            }
            "InputAudio" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputAudio(input_num, is_active))
            }
            "InputSolo" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputSolo(input_num, is_active))
            }
            "InputBusAAudio" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputBusAAudio(input_num, is_active))
            }
            "InputBusBAudio" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputBusBAudio(input_num, is_active))
            }
            "InputMasterAudio" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMasterAudio(input_num, is_active))
            }
            "MasterAudio" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::MasterAudio(is_active))
            }
            "BusAAudio" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::BusAAudio(is_active))
            }
            "BusBAudio" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::BusBAudio(is_active))
            }
            "FadeToBlack" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::FadeToBlack(is_active))
            }
            "Recording" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::Recording(is_active))
            }
            "Streaming" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::Streaming(is_active))
            }
            "External" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::External(is_active))
            }
            "Fullscreen" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::Fullscreen(is_active))
            }
            _ => Err(anyhow::anyhow!("Unknown Activator")),
        }
    }
}

pub enum Command {
    TALLY(TallyResponse),
    FUNCTION(FunctionResponse),
    ACTS(ActivatorsResponse),
    XML(XMLResponse),
    XMLTEXT(XMLTextResponse),
    SUBSCRIBE(SubscribeResponse),
    UNSUBSCRIBE(UnsubscribeResponse),
    QUIT,
    VERSION(VersionResponse),
}

// TODO: add "Request" struct

impl TryFrom<&TcpStream> for Command {
    type Error = anyhow::Error;

    fn try_from(stream: &TcpStream) -> std::result::Result<Self, Self::Error> {
        let mut stream = BufReader::new(stream);

        // read stream
        let mut value = String::new();
        stream.read_line(&mut value)?;

        // println!("DEBUG RECEIVED LINE: {:?}", value);

        // remove \r\n
        let value = value.lines().collect::<String>();

        let mut commands: Vec<String> = vec![];
        let mut iter = value.split_whitespace();
        loop {
            if let Some(command) = iter.next() {
                commands.push(command.to_string());
            } else {
                break;
            }
        }

        // first element
        let binding: &String = commands.get(0).unwrap();
        let command = binding.as_str().try_into()?;
        let status: Status = commands.get(1).unwrap().to_owned().into();
        let body: Option<String> = commands.get(2).cloned();
        match command {
            // Example Response: TALLY OK 0121...\r\n
            "TALLY" => {
                let mut tally_map = HashMap::new();
                // check if status is ok
                let chars: Vec<char> = body.unwrap().chars().collect::<Vec<char>>();
                for (i, char) in chars.iter().enumerate() {
                    let tally: TallyData = (*char).into();
                    let mut index = i as InputNumber;
                    index += 1;
                    tally_map.insert(index, tally);
                }
                Ok(Self::TALLY(TallyResponse {
                    status,
                    body: tally_map,
                }))
            }
            // Example Response: FUNCTION OK PreviewInput\r\n
            // Example Response: FUNCTION ER Error message\r\n
            "FUNCTION" => Ok(Self::FUNCTION(FunctionResponse { status, body })),
            // Example Response: ACTS OK Input 1 1\r\n
            "ACTS" => {
                // 2以降のベクターを使用する
                let len = commands.len();
                let raw = &commands.clone()[2..len];
                let body = ActivatorsData::try_from(raw).unwrap();
                Ok(Self::ACTS(ActivatorsResponse { status, body }))
            }
            /*
            Example Response: XML 37\r\n
            <vmix><version>x.x.x.x</version></vmix>
            */
            "XML" => {
                if let Length(len) = &status {
                    let mut took = stream.take(len.to_owned());
                    let mut xml = String::new();
                    took.read_to_string(&mut xml)?;

                    // remove \r\n
                    let xml = xml.lines().collect::<String>();

                    let vmix: Vmix = serde_xml_rs::from_str(xml.as_str()).unwrap();
                    return Ok(Self::XML(XMLResponse { status, body: vmix }));
                }
                Err(anyhow::anyhow!("Failed to read XML"))
            }
            "XMLTEXT" => Ok(Self::XMLTEXT(XMLTextResponse { status, body })),
            "SUBSCRIBE" => Ok(Self::SUBSCRIBE(SubscribeResponse { status, body })),
            "UNSUBSCRIBE" => Ok(Self::UNSUBSCRIBE(UnsubscribeResponse { status, body })),
            "QUIT" => Ok(Self::QUIT), // No body
            "VERSION" => Ok(Self::VERSION(VersionResponse {
                status,
                version: body,
            })),
            _ => Err(anyhow::anyhow!("No matching command found")),
        }
    }
}

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
            // TODO: 切断されているか確認する
            let read_result: Result<Command, _> = reader.try_into();
            if let Err(err) = read_result {
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

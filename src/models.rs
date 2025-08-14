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

    #[serde(rename = "busA")]
    bus_a: Master,

    #[serde(rename = "busB")]
    bus_b: Master,
}

#[derive(Serialize, Deserialize)]
pub struct Master {
    #[serde(rename = "@volume")]
    volume: String,

    #[serde(rename = "@muted")]
    muted: Boolean,

    #[serde(rename = "@meterF1")]
    meter_f1: String,

    #[serde(rename = "@meterF2")]
    meter_f2: String,

    #[serde(rename = "@headphonesVolume", default)]
    headphones_volume: Option<String>,

    #[serde(rename = "@solo", default)]
    solo: Option<Boolean>,

    #[serde(rename = "@sendToMaster", default)]
    send_to_master: Option<Boolean>,
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
    // 属性
    #[serde(rename = "@key")]
    pub key: String,

    #[serde(rename = "@number")]
    pub number: String,

    #[serde(rename = "@type")]
    pub input_type: String,

    #[serde(rename = "@title")]
    pub title: String,

    #[serde(rename = "@shortTitle")]
    pub short_title: String,

    #[serde(rename = "@state", default)]
    pub state: State,

    #[serde(rename = "@position")]
    pub position: String,

    #[serde(rename = "@duration")]
    pub duration: String,

    #[serde(rename = "@loop")]
    pub input_loop: Boolean,

    #[serde(rename = "@muted", default)]
    pub muted: Option<Boolean>,

    #[serde(rename = "@volume", default)]
    pub volume: Option<String>,

    #[serde(rename = "@balance", default)]
    pub balance: Option<String>,

    #[serde(rename = "@solo", default)]
    pub solo: Option<Boolean>,

    #[serde(rename = "@soloPFL", default)]
    pub solo_pfl: Option<Boolean>,

    #[serde(rename = "@audiobusses", default)]
    pub audiobusses: Option<String>,

    #[serde(rename = "@meterF1", default)]
    pub meter_f1: Option<String>,

    #[serde(rename = "@meterF2", default)]
    pub meter_f2: Option<String>,

    #[serde(rename = "@gainDb", default)]
    pub gain_db: Option<String>,

    // 子要素
    #[serde(rename = "list", default)]
    pub list: Option<List>,

    #[serde(rename = "selectedIndex", default)]
    pub selected_index: Option<String>,

    #[serde(rename = "image", default)]
    pub image: Option<Image>,

    #[serde(rename = "replay", default)]
    pub replay: Option<Replay>,

    // テキストコンテンツ（要素の中身）
    #[serde(rename = "$value", default)]
    pub text_content: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    #[serde(rename = "item", default)]
    pub item: Vec<ListItem>,
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
    // 子要素
    #[serde(rename = "timecode", default)]
    pub timecode: String,

    #[serde(rename = "timecodeA", default)]
    pub timecode_a: String,

    #[serde(rename = "timecodeB", default)]
    pub timecode_b: String,

    // 属性
    #[serde(rename = "@live", default)]
    pub live: Option<String>,

    #[serde(rename = "@recording", default)]
    pub recording: Option<String>,

    #[serde(rename = "@channelMode", default)]
    pub channel_mode: String,

    #[serde(rename = "@events", default)]
    pub events: String,

    #[serde(rename = "@eventsA", default)]
    pub events_a: String,

    #[serde(rename = "@eventsB", default)]
    pub events_b: String,

    #[serde(rename = "@cameraA", default)]
    pub camera_a: String,

    #[serde(rename = "@cameraB", default)]
    pub camera_b: String,

    #[serde(rename = "@speed", default)]
    pub speed: String,

    #[serde(rename = "@speedA", default)]
    pub speed_a: String,

    #[serde(rename = "@speedB", default)]
    pub speed_b: String,
}

#[derive(Serialize, Deserialize)]
pub struct Overlays {
    #[serde(rename = "overlay")]
    overlay: Vec<OverlaysOverlay>,
}

#[derive(Serialize, Deserialize)]
pub struct OverlaysOverlay {
    #[serde(rename = "@number")]
    number: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transitions {
    #[serde(rename = "transition")]
    transition: Vec<Transition>,
}

#[derive(Serialize, Deserialize)]
pub struct Transition {
    #[serde(rename = "@number")]
    number: String,

    #[serde(rename = "@effect")]
    effect: String,

    #[serde(rename = "@duration")]
    duration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListItem {
    #[serde(rename = "@enabled", default)]
    pub enabled: Option<String>,
    #[serde(rename = "@selected", default)]
    pub selected: Option<String>,
    #[serde(rename = "$value", default)]
    pub text: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum State {
    #[serde(rename = "Paused")]
    Paused,

    #[serde(rename = "Running")]
    Running,

    #[serde(rename = "Completed")]
    Completed,

    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for State {
    fn default() -> Self {
        State::Unknown
    }
}

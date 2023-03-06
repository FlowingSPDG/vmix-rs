use crate::acts::ActivatorsData;
use crate::commands::Status::Length;
use crate::models::Vmix;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

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
            '0' => TallyData::OFF,
            '1' => TallyData::PROGRAM,
            '2' => TallyData::PREVIEW,
            _ => TallyData::OFF, // NO MATCHING PATTERN
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

impl TryFrom<&TcpStream> for Command {
    type Error = anyhow::Error;

    fn try_from(stream: &TcpStream) -> Result<Self, Self::Error> {
        let mut stream = BufReader::new(stream);

        // read stream
        let mut value = String::new();

        // start reading buffer
        // no data found
        if stream.read_line(&mut value)? == 0 {
            return Err(anyhow::anyhow!(std::io::ErrorKind::ConnectionAborted));
        };

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

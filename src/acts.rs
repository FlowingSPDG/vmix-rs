use crate::commands::InputNumber;

#[derive(Debug)]
pub enum ActivatorsData {
    Input(InputNumber, bool),
    InputMix2(InputNumber, bool),
    InputMix3(InputNumber, bool),
    InputMix4(InputNumber, bool),
    InputMix5(InputNumber, bool),
    InputMix6(InputNumber, bool),
    InputMix7(InputNumber, bool),
    InputMix8(InputNumber, bool),
    InputMix9(InputNumber, bool),
    InputMix10(InputNumber, bool),
    InputMix11(InputNumber, bool),
    InputMix12(InputNumber, bool),
    InputMix13(InputNumber, bool),
    InputMix14(InputNumber, bool),
    InputMix15(InputNumber, bool),
    InputMix16(InputNumber, bool),
    InputPreview(InputNumber, bool),
    InputPreviewMix2(InputNumber, bool),
    InputPreviewMix3(InputNumber, bool),
    InputPreviewMix4(InputNumber, bool),
    InputPreviewMix5(InputNumber, bool),
    InputPreviewMix6(InputNumber, bool),
    InputPreviewMix7(InputNumber, bool),
    InputPreviewMix8(InputNumber, bool),
    InputPreviewMix9(InputNumber, bool),
    InputPreviewMix10(InputNumber, bool),
    InputPreviewMix11(InputNumber, bool),
    InputPreviewMix12(InputNumber, bool),
    InputPreviewMix13(InputNumber, bool),
    InputPreviewMix14(InputNumber, bool),
    InputPreviewMix15(InputNumber, bool),
    InputPreviewMix16(InputNumber, bool),
    InputPlaying(InputNumber, bool),
    InputVolume(InputNumber, f32),
    InputHeadphones(InputNumber, f32),
    MasterVolume(f32),
    MasterHeadphones(f32),
    BusAVolume(f32),
    BusBVolume(f32),
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
    Overlay1(InputNumber, bool),
    Overlay2(InputNumber, bool),
    Overlay3(InputNumber, bool),
    Overlay4(InputNumber, bool),
    ReplayPlaying(bool),
}

impl TryFrom<&[String]> for ActivatorsData {
    type Error = anyhow::Error;
    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value[0].as_str() {
            "Input" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::Input(input_num, is_active))
            }
            "InputMix2" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix2(input_num, is_active))
            }
            "InputMix3" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix3(input_num, is_active))
            }
            "InputMix4" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix4(input_num, is_active))
            }
            "InputMix5" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix5(input_num, is_active))
            }
            "InputMix6" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix6(input_num, is_active))
            }
            "InputMix7" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix7(input_num, is_active))
            }
            "InputMix8" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix8(input_num, is_active))
            }
            "InputMix9" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix9(input_num, is_active))
            }
            "InputMix10" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix10(input_num, is_active))
            }
            "InputMix11" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix11(input_num, is_active))
            }
            "InputMix12" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix12(input_num, is_active))
            }
            "InputMix13" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix13(input_num, is_active))
            }
            "InputMix14" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix14(input_num, is_active))
            }
            "InputMix15" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix15(input_num, is_active))
            }
            "InputMix16" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputMix16(input_num, is_active))
            }
            "InputPreview" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreview(input_num, is_active))
            }
            "InputPreviewMix2" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix2(input_num, is_active))
            }
            "InputPreviewMix3" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix3(input_num, is_active))
            }
            "InputPreviewMix4" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix4(input_num, is_active))
            }
            "InputPreviewMix5" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix5(input_num, is_active))
            }
            "InputPreviewMix6" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix6(input_num, is_active))
            }
            "InputPreviewMix7" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix7(input_num, is_active))
            }
            "InputPreviewMix8" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix8(input_num, is_active))
            }
            "InputPreviewMix9" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix9(input_num, is_active))
            }
            "InputPreviewMix10" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix10(input_num, is_active))
            }
            "InputPreviewMix11" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix11(input_num, is_active))
            }
            "InputPreviewMix12" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix12(input_num, is_active))
            }
            "InputPreviewMix13" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix13(input_num, is_active))
            }
            "InputPreviewMix14" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix14(input_num, is_active))
            }
            "InputPreviewMix15" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix15(input_num, is_active))
            }
            "InputPreviewMix16" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPreviewMix16(input_num, is_active))
            }
            "InputPlaying" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::InputPlaying(input_num, is_active))
            }
            "InputVolume" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let volume = value[2].parse::<f32>().unwrap();
                Ok(ActivatorsData::InputVolume(input_num, volume))
            }
            "InputHeadphones" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let volume = value[2].parse::<f32>().unwrap();
                Ok(ActivatorsData::InputHeadphones(input_num, volume))
            }
            "MasterVolume" => {
                let volume = value[1].parse::<f32>().unwrap();
                Ok(ActivatorsData::MasterVolume(volume))
            }
            "MasterHeadphones" => {
                let volume = value[1].parse::<f32>().unwrap();
                Ok(ActivatorsData::MasterHeadphones(volume))
            }
            "BusAVolume" => {
                let volume = value[1].parse::<f32>().unwrap();
                Ok(ActivatorsData::BusAVolume(volume))
            }
            "BusBVolume" => {
                let volume = value[1].parse::<f32>().unwrap();
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
            "Overlay1" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::Overlay1(input_num, is_active))
            }
            "Overlay2" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::Overlay2(input_num, is_active))
            }
            "Overlay3" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::Overlay3(input_num, is_active))
            }
            "Overlay4" => {
                let input_num = value[1].parse::<InputNumber>().unwrap();
                let is_active = value[2].as_str() == "1";
                Ok(ActivatorsData::Overlay4(input_num, is_active))
            }
            "ReplayPlaying" => {
                let is_active = value[1].as_str() == "1";
                Ok(ActivatorsData::ReplayPlaying(is_active))
            }
            _ => Err(anyhow::anyhow!("Unknown Activator")),
        }
    }
}

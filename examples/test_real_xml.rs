use quick_xml::de;
use vmix_rs::models::Vmix;

fn main() {
    let xml_content = r#"<vmix><version>27.0.0.81</version><edition>4K</edition><preset>C:\Users\RAGE-003\Downloads\TKL AudioMix\LTK Playoff rev3.vmix</preset><inputs><input key="0bb3d8ac-15c9-4b6d-93c8-89ac76d357c5" number="1" type="Colour" title="Team A MIX" shortTitle="Team A MIX" state="Paused" position="0" duration="0" loop="False">Team A MIX<overlay index="0" key="e851882d-26da-417d-a1de-3f78b79b156b" /><overlay index="1" key="52034e76-569e-4e09-808e-6697c461ead6"><position panX="-0.562" panY="-0.8" zoomX="0.2" zoomY="0.2" x="228.5" y="864" width="384" height="216" /></overlay><overlay index="2" key="a536e112-48a0-4a58-803b-758eb6689387"><position panX="-0.28" panY="-0.8" zoomX="0.2" zoomY="0.2" x="499.2" y="864" width="384" height="216" /></overlay><overlay index="3" key="bbab5b14-82aa-4f18-ba96-3e8903ab12c5"><position panX="0.003" panY="-0.8" zoomX="0.2" zoomY="0.2" x="770.9" y="864" width="384" height="216" /></overlay><overlay index="4" key="e4cce7e1-e188-453e-b346-e8f8c3ed7be6"><position panX="0.287" panY="-0.8" zoomX="0.2" zoomY="0.2" x="1043.5" y="864" width="384" height="216" /></overlay><overlay index="5" key="6d879c84-92f4-474f-a0fa-7221634837d1"><position panX="0.568" panY="-0.8" zoomX="0.2" zoomY="0.2" x="1313.3" y="864" width="384" height="216" /></overlay><overlay index="6" key="dc151ef7-fb96-4592-bde8-b474d8c754a9" /></input><input key="962a66c8-885c-4c79-b452-30f80a0f5adf" number="21" type="VideoList" title="Left_Top - DC CORE_Top.png" shortTitle="Left_Top" state="Paused" position="0" duration="0" loop="False" muted="False" volume="100" balance="0" solo="False" soloPFL="False" audiobusses="M" meterF1="0" meterF2="0" gainDb="0" selectedIndex="1">Left_Top - DC CORE_Top.png<list><item selected="true">C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\DC CORE_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\DC NEXT_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\PD CORE_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\PD NEXT_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\RR CORE_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\RR NEXT_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\ST CORE_Top.png</item><item>C:\Users\RAGE-003\Downloads\TKL AudioMix\Playoff Camera\1_Top\ST NEXT_Top.png</item></list></input></inputs><overlays><overlay number="1" /><overlay number="2" /><overlay number="3" /><overlay number="4" /><overlay number="5" /><overlay number="6" /><overlay number="7" /><overlay number="8" /></overlays><preview>1</preview><active>1</active><fadeToBlack>False</fadeToBlack><transitions><transition number="1" effect="Fade" duration="500" /><transition number="2" effect="Merge" duration="1000" /><transition number="3" effect="Wipe" duration="1000" /><transition number="4" effect="CubeZoom" duration="1000" /></transitions><recording>False</recording><external>True</external><streaming>False</streaming><playList>False</playList><multiCorder>False</multiCorder><fullscreen>False</fullscreen><audio><master volume="100" muted="False" meterF1="1.229964E-05" meterF2="1.229964E-05" headphonesVolume="100" /><busA volume="59.96953" muted="False" meterF1="2.477735E-05" meterF2="2.108933E-05" solo="False" sendToMaster="False" /><busB volume="59.96953" muted="False" meterF1="2.277565E-05" meterF2="2.277565E-05" solo="False" sendToMaster="False" /></audio><dynamic><input1></input1><input2></input2><input3></input3><input4></input4><value1></value1><value2></value2><value3></value3><value4></value4></dynamic></vmix>"#;

    match de::from_str::<Vmix>(xml_content) {
        Ok(vmix) => {
            println!("✅ Successfully parsed vMix XML!");
            println!("Version: {}", vmix.version);
            println!("Edition: {}", vmix.edition);
            println!("Total inputs: {}", vmix.inputs.input.len());

            // Check VideoList inputs
            let video_lists: Vec<_> = vmix
                .inputs
                .input
                .iter()
                .filter(|input| input.input_type == "VideoList")
                .collect();
            println!("VideoList inputs: {}", video_lists.len());

            for input in video_lists {
                if let Some(list) = &input.list {
                    println!("  Input {}: {} items", input.number, list.item.len());
                    for (i, item) in list.item.iter().take(3).enumerate() {
                        println!(
                            "    Item {}: selected={:?}, text={:?}",
                            i, item.selected, item.text
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to parse vMix XML: {}", e);
            eprintln!("Error details: {:?}", e);
        }
    }
}

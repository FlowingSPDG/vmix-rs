use vmix_rs::models::Input;
use quick_xml::de;

#[test]
fn test_image_parsing_without_image_element() {
    // image要素が存在しない場合のテスト
    let xml_without_image = r#"<input key="test-key" number="1" type="Capture" title="Test" shortTitle="Test" state="Running" position="0" duration="0" loop="False">Test</input>"#;
    
    let result: Result<Input, _> = de::from_str(xml_without_image);
    
    match result {
        Ok(input) => {
            println!("✅ Successfully parsed input without image element");
            println!("   image vec length: {}", input.image.len());
            assert_eq!(input.image.len(), 0, "image should be empty Vec when element is missing");
        }
        Err(e) => {
            println!("❌ Failed to parse input without image element: {}", e);
            panic!("Parsing failed: {}", e);
        }
    }
}

#[test]
fn test_image_parsing_with_single_image_element() {
    // image要素が1つだけ存在する場合のテスト
    let xml_with_single_image = r#"<input key="test-key" number="1" type="Images" title="Test" shortTitle="Test" state="Running" position="0" duration="0" loop="False">
        <image>
            <index>0</index>
            <name>test.jpg</name>
        </image>
    </input>"#;
    
    let result: Result<Input, _> = de::from_str(xml_with_single_image);
    
    match result {
        Ok(input) => {
            println!("✅ Successfully parsed input with single image element");
            println!("   image vec length: {}", input.image.len());
            assert_eq!(input.image.len(), 1, "image should contain 1 element");
            if let Some(img) = input.image.first() {
                println!("   image index: {:?}, name: {:?}", img.index, img.name);
            }
        }
        Err(e) => {
            println!("❌ Failed to parse input with single image element: {}", e);
            panic!("Parsing failed: {}", e);
        }
    }
}

#[test]
fn test_image_parsing_with_multiple_image_elements() {
    // image要素が複数存在する場合のテスト
    let xml_with_multiple_images = r#"<input key="test-key" number="1" type="Images" title="Test" shortTitle="Test" state="Running" position="0" duration="0" loop="False">
        <image>
            <index>0</index>
            <name>test1.jpg</name>
        </image>
        <image>
            <index>1</index>
            <name>test2.jpg</name>
        </image>
    </input>"#;
    
    let result: Result<Input, _> = de::from_str(xml_with_multiple_images);
    
    match result {
        Ok(input) => {
            println!("✅ Successfully parsed input with multiple image elements");
            println!("   image vec length: {}", input.image.len());
            assert_eq!(input.image.len(), 2, "image should contain 2 elements");
        }
        Err(e) => {
            println!("❌ Failed to parse input with multiple image elements: {}", e);
            panic!("Parsing failed: {}", e);
        }
    }
}

#[test]
fn test_image_parsing_with_empty_image_element() {
    // image要素が空の場合のテスト（XML的には存在するが内容がない）
    let xml_with_empty_image = r#"<input key="test-key" number="1" type="Images" title="Test" shortTitle="Test" state="Running" position="0" duration="0" loop="False">
        <image></image>
    </input>"#;
    
    let result: Result<Input, _> = de::from_str(xml_with_empty_image);
    
    match result {
        Ok(input) => {
            println!("✅ Successfully parsed input with empty image element");
            println!("   image vec length: {}", input.image.len());
            // 空のimage要素でも1つの要素としてパースされる可能性がある
            println!("   Note: Empty image element may still be parsed as a single element");
        }
        Err(e) => {
            println!("❌ Failed to parse input with empty image element: {}", e);
            panic!("Parsing failed: {}", e);
        }
    }
}


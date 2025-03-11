use quickxml_to_serde::{xml_string_to_json, Config};
use serde_json::{to_string_pretty, Value};
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

fn extract_score_xml(mxl_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(mxl_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        if file_name.ends_with(".xml") && file_name != "META-INF/container.xml" {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            return Ok(content);
        }
    }
    Err("No MusicXML (score.xml) file found in MXL archive".into())
}

pub fn convert_mxl_to_json(mxl_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let xml_content = extract_score_xml(mxl_path)?;
    let conf = Config::new_with_defaults();
    let json = xml_string_to_json(xml_content.to_owned(), &conf);
    let json_str = json.expect("Malformed XML").to_string();
    let value: Value = serde_json::from_str(&json_str)?;
    let pretty_json = to_string_pretty(&value)?;
    Ok(pretty_json)
}

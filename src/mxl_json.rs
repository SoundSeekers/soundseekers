use quickxml_to_serde::{xml_string_to_json, Config};
use serde_json::{to_string_pretty, Value};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;

fn read_musicxml_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn extract_score_xml(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    match path.extension().and_then(|s| s.to_str()) {
        Some("mxl") => {
            let file = File::open(file_path)?;
            let mut archive = ZipArchive::new(file)?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                if file.name().ends_with(".xml") && file.name() != "META-INF/container.xml" {
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    return Ok(content);
                }
            }
            Err("No MusicXML (score.xml) file found in MXL archive".into())
        }
        Some("musicxml") => read_musicxml_file(file_path),
        _ => Err("Unsupported file format".into()),
    }
}

pub fn convert_mxl_to_json(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let xml_content = extract_score_xml(file_path)?;
    let conf = Config::new_with_defaults();
    let json = xml_string_to_json(xml_content, &conf);
    let json_str = json.expect("Malformed XML").to_string();
    let value: Value = serde_json::from_str(&json_str)?;
    Ok(to_string_pretty(&value)?)
}

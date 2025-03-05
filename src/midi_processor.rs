use midly::{Smf, TrackEventKind};
use serde::Serialize;
use std::{fs::File, io::{Read}, error::Error};

#[derive(Serialize)]
pub struct MidiJson {
    pub tracks: Vec<Vec<EventJson>>,
}

#[derive(Serialize)]
pub struct EventJson {
    pub delta_time: u32,
    pub event_type: String,
}

pub fn convert_midi_to_json(input_file: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(input_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let smf = Smf::parse(&buffer)?;

    let tracks = smf.tracks.iter().map(|track| {
        track.iter().map(|event| {
            let event_type = match &event.kind {
                TrackEventKind::Midi { message, .. } => format!("{:?}", message),
                TrackEventKind::Meta(meta) => format!("{:?}", meta),
                TrackEventKind::SysEx(data) => format!("SysEx({} bytes)", data.len()),
                _ => "Unknown".to_string(),
            };
            EventJson {
                delta_time: event.delta.as_int(),
                event_type,
            }
        }).collect()
    }).collect();

    let midi_json = MidiJson { tracks };
    let json = serde_json::to_string_pretty(&midi_json)?;

    Ok(json)
}

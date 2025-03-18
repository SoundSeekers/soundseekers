//! # MIDI Processor
//!
//! This module handles various operations for working with MIDI files
//!
//! ## Features
//! - Convert MIDI file to JSON.
//!
//!
//!
//!


use midly::{Smf, Track, TrackEvent, TrackEventKind, MidiMessage, MetaMessage, Header, Format, Timing};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{Read}, error::Error};
use std::fmt::Write;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct MidiJson {
    pub tracks: Vec<Vec<EventJson>>,
}

#[derive(Serialize, Deserialize)]
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


/*
pub fn convert_json_to_midi(json_file: &str) -> Result<Vec<u8>, Box<dyn Error>> {
 let mut file = File::open(json_file)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    let midi_json: MidiJson = serde_json::from_str(&json_str)?;
    let mut tracks = Vec::new();

    for track_json in midi_json.tracks {
        let mut track = Track::new();

        for event in track_json {
            let delta = event.delta_time.into();
            let kind = match event.event_type.as_str() {
                "EndOfTrack" => TrackEventKind::Meta(MetaMessage::EndOfTrack),
                _ if event.event_type.starts_with("TrackName") => {
                    let name = event.event_type[10..].trim_matches(|c| c == '[' || c == ']').to_owned();
                    let name_byte: Cow<'static, [u8]> = Cow::Owned(name.into_bytes());
                    TrackEventKind::Meta(MetaMessage::TrackName(name_byte.as_ref()))
                }
                _ if event.event_type.starts_with("TimeSignature") => {
                    let parts: Vec<u8> = event.event_type[14..]
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if parts.len() == 4 {
                        TrackEventKind::Meta(MetaMessage::TimeSignature(parts[0], parts[1], parts[2], parts[3]))
                    } else {
                        continue;
                    }
                }
                _ if event.event_type.starts_with("KeySignature") => {
                    let parts: Vec<&str> = event.event_type[12..].trim_matches(|c| c == '(' || c == ')').split(',').collect();
                    if let (Ok(key), minor) = (parts[0].parse::<i8>(), parts[1] == "true") {
                        TrackEventKind::Meta(MetaMessage::KeySignature(key, minor))
                    } else {
                        continue;
                    }
                }
                _ if event.event_type.starts_with("Tempo") => {
                    if let Ok(tempo) = event.event_type[6..].trim_matches(|c| c == '(' || c == ')').parse::<u32>() {
                        TrackEventKind::Meta(MetaMessage::Tempo(tempo.into()))
                    } else {
                        continue;
                    }
                }
                _ if event.event_type.starts_with("NoteOn") => {
                    let parts: Vec<u8> = event.event_type[7..]
                        .trim_matches(|c| c == '{' || c == '}')
                        .split(',')
                        .filter_map(|s| s.split(':').nth(1)?.trim().parse().ok())
                        .collect();
                    if parts.len() == 2 {
                        TrackEventKind::Midi {
                            channel: 0.into(),
                            message: MidiMessage::NoteOn { key: parts[0].into(), vel: parts[1].into() },
                        }
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            track.push(TrackEvent { delta, kind });
        }

        tracks.push(track);
    }

    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(480.into()),
        },
        tracks,
    };

    let mut midi_data = Vec::new();
    smf.write(&mut midi_data)?;
    Ok(midi_data)
}
*/

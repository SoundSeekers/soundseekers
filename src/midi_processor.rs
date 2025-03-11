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

pub fn convert_json_to_midi(json_file: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(json_file)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    let midi_json: MidiJson = serde_json::from_str(&json_str)?;
    let mut tracks = Vec::new();

    // Process each track in the MIDI JSON
    for track_json in midi_json.tracks {
        let mut track = Track::new();

        // Process each event in the track
        for event in track_json {
            let delta = event.delta_time.into();

            // Parse the event type and its parameters
            let kind = if let Some(event) = event.event_type.strip_prefix("TrackName") {
                let track_name = event.trim_matches(|c| c == '[' || c == ']');
                TrackEventKind::Meta(MetaMessage::TrackName(track_name.as_bytes()))
            } else if let Some(event) = event.event_type.strip_prefix("TimeSignature") {
                let parts: Vec<&str> = event.trim_matches(|c| c == '(' || c == ')').split(',').collect();
                let numerator = parts[0].parse::<u8>()?;
                let denominator = parts[1].parse::<u8>()?;
                let clocks_per_metronome = parts[2].parse::<u8>()?;
                let thirtyseconds_per_quarter = parts[3].parse::<u8>()?;
                TrackEventKind::Meta(MetaMessage::TimeSignature(numerator, denominator, clocks_per_metronome, thirtyseconds_per_quarter))
            } else if let Some(event) = event.event_type.strip_prefix("KeySignature") {
                let parts: Vec<&str> = event.trim_matches(|c| c == '(' || c == ')').split(',').collect();
                let key = parts[0].parse::<i8>()?;
                let minor = parts[1] == "true";
                TrackEventKind::Meta(MetaMessage::KeySignature(key, minor))
            } else if let Some(event) = event.event_type.strip_prefix("Tempo") {
                let tempo = event.trim_matches(|c| c == 'u' || c == '2' || c == '(' || c == ')')
                    .parse::<u32>()?;
                TrackEventKind::Meta(MetaMessage::Tempo(tempo.into()))
            } else if let Some(event) = event.event_type.strip_prefix("Controller") {
                let parts: Vec<&str> = event.trim_matches(|c| c == '{' || c == '}').split(',').collect();
                let controller = parts[0].trim().parse::<u8>()?;
                let value = parts[1].trim().parse::<u8>()?;
                TrackEventKind::Midi {
                    channel: 0.into(),
                    message: MidiMessage::ControlChange {
                        controller: controller.into(),
                        value: value.into(),
                    },
                }
            } else if let Some(event) = event.event_type.strip_prefix("ProgramChange") {
                let program = event.trim_matches(|c| c == '{' || c == '}').split(":").collect::<Vec<&str>>()[1].trim().parse::<u8>()?;
                TrackEventKind::Midi {
                    channel: 0.into(),
                    message: MidiMessage::ProgramChange { program: program.into() },
                }
            } else if let Some(event) = event.event_type.strip_prefix("NoteOn") {
                let parts: Vec<&str> = event.trim_matches(|c| c == '{' || c == '}').split(',').collect();
                let key = parts[0].trim().split(":").collect::<Vec<&str>>()[1].trim().parse::<u8>()?;
                let velocity = parts[1].trim().split(":").collect::<Vec<&str>>()[1].trim().parse::<u8>()?;
                TrackEventKind::Midi {
                    channel: 0.into(),
                    message: MidiMessage::NoteOn {
                        key: key.into(),
                        vel: velocity.into(),
                    },
                }
            } else if event.event_type == "EndOfTrack" {
                TrackEventKind::Meta(MetaMessage::EndOfTrack)
            } else {
                continue; // Skip unsupported events
            };

            // Add the event to the track
            track.push(TrackEvent { delta, kind });
        }

        // Add the track to the overall list of tracks
        tracks.push(track);
    }

    // Create the SMF (Standard MIDI File) structure
    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(480.into()),
        },
        tracks,
    };

    // Write the smf structure into a string for output
    let mut midi_string = String::new();
    write!(midi_string, "{:?}", smf)?;

    // Return the MIDI string
    Ok(midi_string)
}

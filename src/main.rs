
use midly::{Smf, TrackEventKind};
use serde::Serialize;
use std::{env, fs::File, io::Read};

#[derive(Serialize)]
struct MidiJson {
    tracks: Vec<Vec<EventJson>>,

}

#[derive(Serialize)]
struct EventJson {
    delta_time: u32,
    event_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Look for an argument that starts with 'if='
    let input_file = args.iter()
        .find(|arg| arg.starts_with("if="))
        .map(|arg| arg.trim_start_matches("if="))
        .ok_or("Usage: program if=path/to/file.mid")?;

    let mut file = File::open(input_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let smf = Smf::parse(&buffer)?;

    let tracks = smf.tracks.iter().map(|track| {
        track.iter().map(|event| {
            let event_type = match &event.kind {
                TrackEventKind::Midi { message, .. } => format!("{:?}", message),
                TrackEventKind::Meta(meta) => format!("{:?}", meta),
                TrackEventKind::SysEx(data) => format!("SysEx({}) bytes", data.len()),
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
    println!("{}", json);

    Ok(())



}

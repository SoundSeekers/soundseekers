use clap::{App, Arg, SubCommand};
use midi_processor::{convert_midi_to_json, convert_json_to_midi};
use mxl_json::{convert_mxl_to_json};
use std::process::exit;

mod midi_processor;
mod mxl_json;

fn main() {
    // Define the main app and its subcommands using clap
    let matches = App::new("Soundseekers")
        .version("1.0")
        .author("Felix")
        .about("A tool to process MIDI files")
        .subcommand(
            SubCommand::with_name("midi")
                .about("MIDI related operations")
                .subcommand(
                    SubCommand::with_name("convert")
                        .about("Convert MIDI to JSON format")
                        .subcommand(
                            SubCommand::with_name("json")
                                .about("Convert to JSON format")
                                .arg(
                                    Arg::new("input")
                                        .long("if")
                                        .takes_value(true)
                                        .required(true)
                                        .help("Input MIDI file path"),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name("midi")
                                .about("Convert JSON to MIDI format")
                                .arg(
                                    Arg::new("input")
                                        .long("if")
                                        .takes_value(true)
                                        .required(true)
                                        .help("Input JSON file path"),
                                    ),
                            ),

                ),
        )
        .subcommand(
            SubCommand::with_name("mxl")
                .about("MusicXML related operations")
                .subcommand(
                    SubCommand::with_name("convert")
                        .about("Convert MusicXML files")
                        .subcommand(
                            SubCommand::with_name("json")
                                .about("Convert to JSON format")
                                .arg(
                                    Arg::new("input")
                                        .long("if")
                                        .takes_value(true)
                                        .required(true)
                                        .help("Input MusicXML file path"),
                                ),
                        ),
                ),
        )
        .get_matches();

    // Handle the "midi" subcommand
    if let Some(midi_matches) = matches.subcommand_matches("midi") {
        if let Some(convert_matches) = midi_matches.subcommand_matches("convert") {
            if let Some(json_matches) = convert_matches.subcommand_matches("json") {
                if let Some(input_file) = json_matches.value_of("input") {
                    match convert_midi_to_json(input_file) {
                        Ok(json) => println!("{}", json),
                        Err(err) => {
                            eprintln!("Error processing MIDI file: {}", err);
                            exit(1);
                        }
                    }
                }
            }
            if let Some(midi_matches) = convert_matches.subcommand_matches("midi") {
                if let Some(input_file) = midi_matches.value_of("input") {
                    match convert_json_to_midi(input_file) {
                        Ok(midi_data) => println!("{}", midi_data),
                        Err(err) => {
                            eprintln!("Error processing JSON file. {}", err);
                            exit(1);
                        }
                }
            }



        }
    }
    if let Some(mxml_matches) = matches.subcommand_matches("mxl") {
        if let Some(convert_matches) = mxml_matches.subcommand_matches("convert") {
            if let Some(json_matches) = convert_matches.subcommand_matches("json") {
                if let Some(input_file) = json_matches.value_of("input") {
                    match convert_mxl_to_json(input_file) {
                        Ok(json) => println!("{}", json),
                        Err(err) => {
                            eprintln!("Error processing MusicXML file: {}", err);
                            exit(1);
                        }
                    }
                }
            }
        }
    }
}
}

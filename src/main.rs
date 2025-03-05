use clap::{App, Arg, SubCommand};
use midi_processor::convert_midi_to_json;
use std::process::exit;

mod midi_processor;

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
                        .about("Convert MIDI files")
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
        }
    }
}

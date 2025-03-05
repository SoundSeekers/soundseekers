use assert_cmd::Command;
use std::fs;
use predicates::str::contains;

#[test]
fn test_midi_convert_json() {
    let midi_path = "tests/notimeforcaution.mid";

    // Ensure file exists
    assert!(fs::metadata(midi_path).is_ok(), "Test MIDI file is missing");

    let mut cmd = Command::cargo_bin("soundseekers").unwrap();

    cmd.args([
        "midi",
        "convert",
        "json",
        &format!("--if={}", midi_path),
    ]);

    cmd.assert()
        .success()
        .stdout(contains("\"tracks\""));
}

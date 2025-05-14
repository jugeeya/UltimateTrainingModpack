use anyhow::{Context, Result};
use serde_json::{self, Value};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

pub fn extract_smash_open_devices(input_path: &Path, output_path: &Path) -> Result<()> {
    // Open and read the input file
    let file = File::open(input_path)
        .with_context(|| format!("Failed to open input file: {}", input_path.display()))?;
    let reader = BufReader::new(file);

    // Parse the JSON data
    let data: Value =
        serde_json::from_reader(reader).with_context(|| "Failed to parse input JSON")?;

    // Extract all events from the nested structure
    let mut flattened_devices = Vec::new();

    // Navigate through the nested structure: device -> device_id -> timestamp -> event_id -> event_data
    if let Some(devices) = data["device"].as_object() {
        for (_device_id, timestamps) in devices {
            if let Some(timestamps) = timestamps.as_object() {
                for (_timestamp, events) in timestamps {
                    if let Some(events) = events.as_object() {
                        for (_event_id, event_data) in events {
                            flattened_devices.push(event_data.clone());
                        }
                    }
                }
            }
        }
    }

    // Write each record as a separate line of JSON (NDJSON format)
    let output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
    let mut writer = BufWriter::new(output_file);

    for device in &flattened_devices {
        serde_json::to_writer(&mut writer, device)?;
        writer.write_all(b"\n")?;
    }

    println!(
        "Extracted {} records to {}",
        flattened_devices.len(),
        output_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_extract_smash_open_devices() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.json");
        let output_path = temp_dir.path().join("output.json");

        // Create test input JSON with the correct nested structure
        let test_data = r#"{
            "device": {
                "device1": {
                    "1000": {
                        "event1": {
                            "device_id": "device1",
                            "event_name": "SMASH_OPEN",
                            "event_time": 1000,
                            "test": "data1"
                        }
                    }
                },
                "device2": {
                    "2000": {
                        "event2": {
                            "device_id": "device2",
                            "event_name": "SMASH_OPEN",
                            "event_time": 2000,
                            "test": "data2"
                        }
                    }
                }
            }
        }"#;
        std::fs::write(&input_path, test_data).unwrap();

        // Run the extraction
        extract_smash_open_devices(&input_path, &output_path).unwrap();

        // Verify output
        let output_content = std::fs::read_to_string(&output_path).unwrap();
        let output_json: Value = serde_json::from_str(&output_content).unwrap();
        let output_array = output_json.as_array().unwrap();

        assert_eq!(output_array.len(), 2);
        assert_eq!(output_array[0]["test"], "data1");
        assert_eq!(output_array[1]["test"], "data2");
    }
}

use ron::de::from_str;
use serde_json::to_string_pretty;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the filename from command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure there are enough arguments (program name + at least one filename)
    if args.len() < 2 {
        eprintln!("Usage: {} <input.ron>", args[0]);
        std::process::exit(1);
    }

    let input_filename = &args[1];

    // Read the .ron file
    let ron_data = fs::read_to_string(input_filename)?;

    // Parse the RON data
    let parsed: serde_json::Value = from_str(&ron_data)?;

    // Convert the parsed data to JSON and write it to a file
    let output_filename = "output.json"; // You can make this dynamic too, if needed
    let json_data = to_string_pretty(&parsed)?;
    fs::write(output_filename, json_data)?;

    println!("Conversion complete. Output written to {}", output_filename);

    Ok(())
}

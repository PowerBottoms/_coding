// use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Define the path to the file
    let path = "/home/vboxuser/Desktop/nomicexport.txt";

    // Define the target address and termination line
    let target_address = "nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u";
    let termination_line = "min_self_delegation";

    // Open the file
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Initialize state variables
    let mut found_target_count = 0;
    let mut processing = false;
    let mut line_buffer = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;

        // Track occurrences of the target address
        if line.contains(target_address) {
            found_target_count += 1;
            if found_target_count == 2 {
                processing = true;  // Start processing after the second occurrence
                line_buffer.clear(); // Clear the buffer as we're starting a new block
            }
        }

        // Stop processing if the termination line is found
        if processing && line.contains(termination_line) {
            break;
        }

        // Collect lines within the processing block
        if processing {
            line_buffer.push(line.clone());

            // Extract address and shares
            if line.contains("nomic1") {
                if let Some(shares) = extract_shares(&line_buffer) {
                    let address = extract_address(&line).unwrap_or_default();
                    println!("{} shares: {}", address, shares);
                }
                line_buffer.clear(); // Reset buffer for the next address block
            }
        }
    }

    if found_target_count < 2 {
        println!("Target address not found twice in the file.");
    }

    Ok(())
}

// Function to extract the address from a line
fn extract_address(line: &str) -> Option<String> {
    if line.contains("nomic1") {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() > 0 {
            return Some(parts[0].trim().trim_matches('"').to_string());
        }
    }
    None
}

// Function to extract the shares from a buffered set of lines
fn extract_shares(lines: &[String]) -> Option<String> {
    for line in lines {
        if line.contains("shares") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                return Some(parts[1].trim().trim_matches('"').to_string());
            }
        }
    }
    None
}


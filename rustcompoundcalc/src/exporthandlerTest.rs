use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use regex::Regex; // Import the regex crate

// use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "/home/vboxuser/Desktop/Exports/nomicexport.txt";
    let output_file_path = "/home/vboxuser/_coding/datasaves/shares_output.csv";
    let target_address = "nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u";
    let mut found_min_self_delegation = false;
    let mut capturing_addresses = false;

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect all lines in a vector
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut iter = lines.iter().peekable();
    let regex = Regex::new(r"\d+").unwrap(); // Create a regex to match numbers

    // HashMap to store addresses and shares
    let mut address_shares: HashMap<String, f64> = HashMap::new();

    while let Some(line) = iter.next() {
        if line.contains("\"min_self_delegation\"") {
            if capturing_addresses {
                break; // Stop if we encounter a new min_self_delegation after starting to capture addresses
            }
            found_min_self_delegation = true;
        }

        if found_min_self_delegation && line.contains(target_address) {
            found_min_self_delegation = false; // Reset flag
            capturing_addresses = true; // Start capturing subsequent addresses and shares
            continue;
        }

        if capturing_addresses {
            if line.contains("nomic1") {
                // Extract the address from the line
                if let Some(start_idx) = line.find('"') {
                    if let Some(end_idx) = line[start_idx + 1..].find('"') {
                        let address = &line[start_idx + 1..start_idx + end_idx + 1];
                        // Move to the next line twice and check for shares
                        if iter.next().is_some() { // Move down one line
                            if let Some(shares_line) = iter.next() { // Move down the second line
                                if shares_line.contains("\"shares\":") {
                                    // Extract numbers from the shares line
                                    if let Some(captures) = regex.captures(shares_line) {
                                        let shares_value = captures.get(0).unwrap().as_str();
                                        // Convert shares_value to f64 and divide by 1,000,000
                                        if let Ok(shares_num) = shares_value.parse::<f64>() {
                                            let adjusted_shares = shares_num / 1_000_000.0;
                                            // Store address and adjusted shares in the HashMap
                                            address_shares.insert(address.to_string(), adjusted_shares);

                                            // Print the address and shares
                                            println!("Address: {}, Shares: {:.6}", address, adjusted_shares);
                                        } else {
                                            eprintln!("Failed to parse shares value: {}", shares_value);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Write the address and shares to a CSV file
    let mut output_file = File::create(output_file_path)?;
    writeln!(output_file, "Address,Shares")?; // Write header

    for (address, shares) in &address_shares {
        writeln!(output_file, "{},{}", address, shares)?; // Write each address and shares
    }

    Ok(())
}


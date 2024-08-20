 use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex; // Import the regex crate

//use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "/home/vboxuser/Desktop/nomicexport.txt";
    let target_address = "nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u";
    let mut found_min_self_delegation = false;
    let mut capturing_addresses = false;

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Collect all lines in a vector
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut iter = lines.iter().peekable();
    let regex = Regex::new(r"\d+").unwrap(); // Create a regex to match numbers

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
                let address = line.trim().to_string();
                // Move to the next line twice and check for shares
                if iter.next().is_some() { // Move down one line
                    if let Some(shares_line) = iter.next() { // Move down the second line
                        if shares_line.contains("\"shares\":") {
                            // Extract numbers from the shares line
                            if let Some(captures) = regex.captures(shares_line) {
                                let shares_value = captures.get(0).unwrap().as_str();
                                println!("Here are your shares for {}: {}", address, shares_value);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}


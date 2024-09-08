use std::env;
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::exit;

fn main() {
    // Parse the command line argument for number of blocks to check
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./check_blocks <number_of_blocks_to_check>");
        exit(1);
    }

    let blocks_to_check: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: BLOCKS_TO_CHECK must be a positive integer.");
            exit(1);
        }
    };

    // Create an HTTP client
    let client = Client::new();

    // Fetch the validator's address
    let validator_addr = match fetch_validator_address(&client) {
        Some(addr) => addr,
        None => {
            eprintln!("Error: Could not fetch the validator address.");
            exit(1);
        }
    };

    // Fetch the last block height
    let last_block = match fetch_last_block_height(&client) {
        Some(height) => height,
        None => {
            eprintln!("Error: Could not fetch the last block height.");
            exit(1);
        }
    };

    // Calculate the start block
    let start_block = last_block.saturating_sub(blocks_to_check as u64);

    println!("Checking from block {} to {}", start_block, last_block);

    // Initialize missed blocks counter
    let mut missed_blocks = 0;

    // Loop through the blocks and count missed signatures
    for block in start_block..=last_block {
        if !check_block_signature(&client, block, &validator_addr) {
            missed_blocks += 1;
        }
    }

    // Report the final result
    println!(
        "Missed {} block(s) out of {} checked.",
        missed_blocks, blocks_to_check
    );
}

fn fetch_validator_address(client: &Client) -> Option<String> {
    let response = client
        .get("http://localhost:26657/status")
        .send()
        .ok()?
        .json::<Value>()
        .ok()?;

    response["result"]["validator_info"]["address"].as_str().map(|s| s.to_string())
}

fn fetch_last_block_height(client: &Client) -> Option<u64> {
    let response = client
        .get("http://localhost:26657/block")
        .send()
        .ok()?
        .json::<Value>()
        .ok()?;

    response["result"]["block"]["header"]["height"].as_str()?.parse().ok()
}

fn check_block_signature(client: &Client, block: u64, validator_addr: &str) -> bool {
    let url = format!("http://localhost:26657/block?height={}", block);
    let response = client.get(&url).send().ok().and_then(|resp| resp.json::<Value>().ok());

    if let Some(result) = response {
        let empty_vec = vec![]; // Create a persistent empty vector
        let signatures = result["result"]["block"]["last_commit"]["signatures"]
            .as_array()
            .unwrap_or(&empty_vec); // Borrow the persistent empty vector

        let signature = signatures.iter().find(|&sig| {
            sig["validator_address"].as_str() == Some(validator_addr)
        });

        match signature {
            Some(sig) => {
                // If block_id_flag == 2, the block is signed. Otherwise, it's missed.
                return sig["block_id_flag"].as_u64() == Some(2);
            }
            None => return false, // No signature info means missed
        }
    }

    false // Default to missed if no valid response
}


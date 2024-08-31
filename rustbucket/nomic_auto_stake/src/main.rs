use std::process::Command;
use std::str;
use std::thread;
use std::time::Duration;
use chrono::Local;
use colored::*;
fn main() {
	println! ("================================START=============================================================");
	let mut current_value = 0.000000;
	let mut previous_value = 0.000000;
	let mut amntgained = 0.0;
	let threshold = 18.52;
    // Loop indefinitely
    loop {
    	
	let current_time = Local::now();
	let formatted_time = current_time.format("%Y-%b-%a %I:%M:%S %p").to_string(); // Example format: "2024-08-28 03:45:12 PM"
        // Step 1: Execute the `nomic delegations` command
        let output = Command::new("nomic")
            .arg("delegations")
            .output()
            .expect("Failed to execute command");

        // Step 2: Convert output to string and parse the liquid amount
        let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");
        // println!("Output from `nomic delegations`: {}", output_str);  // Debugging output

        // Find the line containing "liquid="
        let liquid_str = output_str
            .lines()
            .find(|line| line.contains("liquid="))
            .expect("No liquid amount found");
	
        // Extract the number directly after "liquid="
        let liquid_amount_str = liquid_str
            .splitn(2, "liquid=")  // Split into two parts at "liquid="
            .nth(1)                // Get the second part (everything after "liquid=")
            .unwrap()
            .split_whitespace()    // Split by whitespace to isolate the number
            .next()                // Get the first segment, which should be the number
            .unwrap()
            .replace(",", "");     // Remove commas if the number has thousand separators

       // println!("Extracted liquid amount string: {}", liquid_amount_str);  // Debugging output

        let claimable_amount: f64 = liquid_amount_str
            .parse()
            .expect("Failed to parse liquid amount");
	
        // Step 3: Divide by 1,000,000
        let formatted_claimable = claimable_amount / 1_000_000.0;
	current_value = formatted_claimable;
	amntgained = current_value - previous_value;	
 	
        if formatted_claimable > threshold {
        let _ = Command::new("nomic")
            .arg("claim")
            .status()  // Use `.status()` instead of `.output()` to avoid capturing output
            .expect("Failed to execute claim command");
            prepare_compound();
        }
       	if previous_value == 0.00000 && formatted_claimable < threshold{
                println!("{} Claimable: {:.6}", formatted_time, formatted_claimable);
        }else if previous_value != 0.00000 && formatted_claimable < threshold{
        	println!("{} Claimable: {:.6}   + {:.6} Since last check", formatted_time, formatted_claimable, amntgained);
        }
	
	println! ("--------------------------------------------------------------------------------------");
	previous_value = current_value;
        thread::sleep(Duration::from_secs(900));
    }

}


fn prepare_compound(){
        ////////NOM BALANCE///////////////////////////////
        // Step 1: Execute the `nomic balance` command
        let bal_output = Command::new("nomic")
            .arg("balance")
            .output()
            .expect("Failed to execute command");

        // Step 2: Convert output to string and parse the liquid amount
        let bal_output_str = str::from_utf8(&bal_output.stdout).expect("Failed to convert to string");
        // println!("Output from `nomic delegations`: {}", output_str);  // Debugging output

        // Find the line containing "NOM"
        let nom_str = bal_output_str
            .lines()
            .find(|line| line.contains("NOM"))
            .expect("No liquid amount found");

        // Extract the number directly after "NOM"
        let nom_bal_str = nom_str
            .splitn(2, "NOM")  // Split into two parts at "liquid="
            .next()                // Get the second part (everything after "liquid=")
            .unwrap()
            .trim()
            .split_whitespace()    // Split by whitespace to isolate the number
            .last()                // Get the first segment, which should be the number
            .unwrap()
            .replace(",", "");     // Remove commas if the number has thousand separators

        // println!("Nomic balance: {}", nom_bal_str);  // Debugging output
         let nom_bal_amount: f64 = nom_bal_str
            .parse()
            .expect("Failed to parse balance amount");
	let delegate_amount = nom_bal_amount - 100_000.0;
    	let _ = Command::new("nomic")
        .arg("delegate")
        .arg("nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u")
        .arg(&delegate_amount.to_string())
        .status()
        .expect("Failed to execute command");
        
        let formatted_delegated: f64 = delegate_amount / 1_000_000.0;
        println!("You have claimed and staked {}", formatted_delegated.to_string().bright_green());
       // println! ("--------------------------------------------------------------------------------------");

}


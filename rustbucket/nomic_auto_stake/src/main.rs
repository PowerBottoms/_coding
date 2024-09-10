use std::process::{Command, exit};
use std::str;
use std::thread;
use std::time::Duration;
use chrono::Local;
use colored::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

//const DIFFS_FILE: &str = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/voting_power_diffs.txt"; // File to store highest differences
fn main() {
    let mut current_value = 0.000000;
    let mut previous_value = 0.000000;
    let mut amntgained = 0.0;
    let triggertime = 1;
    let threshold = 19.48;
    let _bal_output = Command::new("nomic")
     .arg("balance")
     .output()
     .expect("Failed to execute command");

    loop {
        let current_time = Local::now();
        let formatted_time = current_time.format("%I:%M:%S %p   %a-%b-%Y").to_string();

        let output = Command::new("nomic")
            .arg("delegations")
            .output()
            .expect("Failed to execute command");

        let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");

        let liquid_str = match output_str.lines().find(|line| line.contains("liquid=")) {
            Some(liquid_str) => liquid_str,
            None => {
                println!("No liquid amount found. Rerunning the script...");
                Command::new("cargo")
                    .arg("run")
                    .spawn()
                    .expect("Failed to rerun the script");
                exit(0);
            }
        };

        let liquid_amount_str = liquid_str
            .splitn(2, "liquid=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .replace(",", "");

        let claimable_amount: f64 = liquid_amount_str.parse().expect("Failed to parse liquid amount");

        let formatted_claimable = claimable_amount / 1_000_000.0;
        current_value = formatted_claimable;
        amntgained = current_value - previous_value;

        if formatted_claimable > threshold {
            let _ = Command::new("nomic")
                .arg("claim")
                .status()
                .expect("Failed to execute claim command");
            prepare_compound();
        } else if previous_value != 0.00000 && formatted_claimable < threshold {
            println!(
                "{}", format! ("{}   Claimable: {:.6}   Gained: {:.6}",
                formatted_time,
                formatted_claimable,
                amntgained).bright_yellow().on_black()
            );
            let _time_to_claim = calc_tt_claim(formatted_claimable, threshold, amntgained, triggertime);
            println!("{}",format! ("Time to claim {}",_time_to_claim).bright_yellow().on_black());
       	    calculate_voting_power_difference(); 
            println!("{}", "--------------------------------------------------------------------------------------".bright_black().on_black());
        }          
       previous_value = current_value;    
       thread::sleep(Duration::from_secs(triggertime));
  
    }
}


fn prepare_compound(){
        ////////NOM BALANCE///////////////////////////////
        // Step 1: Execute the `nomic balance` command
        let _bal_output = Command::new("nomic")
            .arg("balance")
            .output()
            .expect("Failed to execute command");

        // Step 2: Convert output to string and parse the liquid amount
        let _bal_output_str = str::from_utf8(&_bal_output.stdout).expect("Failed to convert to string");
        // println!("Output from `nomic delegations`: {}", output_str);  // Debugging output

        // Find the line containing "NOM"
        let nom_str = _bal_output_str
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
	thread::sleep(Duration::from_secs(2));
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
 fn calc_tt_claim(claimable_amount: f64, threshold: f64, amntgained: f64, triggertime: u64) -> String {
    if amntgained <= 0.0 {
        return "indefinite".to_string();
    }

    let amount_needed = threshold - claimable_amount;
    let intervals_needed = (amount_needed / amntgained).ceil() as u64;
    let total_seconds = intervals_needed * triggertime;

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02} hours {:02} minutes {:02} seconds", hours, minutes, seconds)
}     




fn calculate_voting_power_difference() {
    let target_address = "nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u";
    let file_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/voting_power_diffs.txt"; // File to save highest voting power differences

    // Load highest voting powers from the file
    let mut highest_differences = vec![0.0; 5]; // Assuming you want to keep track of the last 5 differences
    if Path::new(file_path).exists() {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = io::BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            if i < 5 {
                if let Ok(value) = line {
                    if let Ok(diff) = value.trim().parse::<f64>() {
                        highest_differences[i] = diff;
                    }
                }
            }
        }
    }

    // Run the `nomic validators` command
    let output = Command::new("nomic")
        .arg("validators")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");

    // Find the target address, voting power, and moniker
    let lines = output_str.lines().collect::<Vec<_>>();
    let mut target_voting_power: Option<f64> = None;
    let mut voting_powers: Vec<f64> = Vec::new();
    let mut monikers: Vec<String> = Vec::new(); // Vector to store monikers

    // Look for the target address, its voting power, and moniker
    let mut target_index: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains(target_address) {
            target_index = Some(i);
            if let Some(vp_line) = lines.get(i + 1) {
                if let Some(vp_str) = vp_line.split("VOTING POWER: ").nth(1) {
                    target_voting_power = Some(vp_str.trim().replace(",", "").parse::<f64>().expect("Failed to parse target voting power"));
                }
            }

            if let Some(moniker_line) = lines.get(i + 2) {  // Moniker should be two lines down
                let moniker = moniker_line.replace("MONIKER: ", "");  // Remove "MONIKER: " prefix
                monikers.push(moniker.trim().to_string());
            }

            break; // Stop searching once the target address is found
        }
    }

    // If the target address is found, look back for previous addresses, voting powers, and monikers
    if let Some(index) = target_index {
        let mut look_back_count = 0;
        let mut i = index as isize - 1; // Start looking back from the line before the target address

        while look_back_count < 5 && i >= 0 {
            let line = lines[i as usize];
            if line.contains("nomic1") {
                // Find the voting power line (next line)
                if let Some(vp_line) = lines.get(i as usize + 1) {
                    if let Some(vp_str) = vp_line.split("VOTING POWER: ").nth(1) {
                        let voting_power = vp_str.trim().replace(",", "").parse::<f64>().expect("Failed to parse voting power");
                        voting_powers.push(voting_power);
                    }
                }

                // Capture the moniker line (two lines below the address)
                if let Some(moniker_line) = lines.get(i as usize + 2) {
                    let moniker = moniker_line.replace("MONIKER: ", "");  // Remove "MONIKER: " prefix
                    monikers.push(moniker.trim().to_string());
                }

                look_back_count += 1; // Only increment when a nomic1 address is found
            }
            i -= 1; // Continue looking back
        }
    }

    // If we have the target voting power and at least two previous voting powers, calculate the differences
    if let (Some(vp1), Some(vp2), Some(vp3), Some(vp4), Some(vp5), Some(vp6)) = (
        target_voting_power,
        voting_powers.get(0).cloned(),
        voting_powers.get(1).cloned(),
        voting_powers.get(2).cloned(),
        voting_powers.get(3).cloned(),
        voting_powers.get(4).cloned(),        
    ) {
        let vp1 = vp1 / 1_000_000.0;
        let vp2 = vp2 / 1_000_000.0;
        let vp3 = vp3 / 1_000_000.0;
        let vp4 = vp4 / 1_000_000.0;
        let vp5 = vp5 / 1_000_000.0;
        let vp6 = vp6 / 1_000_000.0;

        // Calculate differences
        let differences = [
            vp2 - vp1,
            vp3 - vp1,
            vp4 - vp1,
            vp5 - vp1,
            vp6 - vp1,
        ];

        // Print differences along with corresponding monikers and highest differences
        for (i, diff) in differences.iter().enumerate() {
            if let Some(moniker) = monikers.get(i + 1) {
                let is_highest = *diff < highest_differences[i];
                if is_highest {
                    highest_differences[i] = *diff; // Update highest difference
                }
                // Print current difference and highest difference next to it
// Print current difference and highest difference next to it
println!(
    "{} | {} | Highest: {} | {}",
    format!("({})", i + 1).yellow(),
    format!("{:.2}",diff).bright_yellow(),
    format! ("{:.2}",highest_differences[i]).green(),
    moniker
);

            }
        }

        // Save the highest differences back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Truncate the file to overwrite
            .open(file_path)
            .expect("Unable to open file for writing");
        for diff in highest_differences.iter() {
            writeln!(file, "{:.2}", diff).expect("Unable to write to file");
        }
    } else {
        println!("Could not find all required voting powers.");
    }
}

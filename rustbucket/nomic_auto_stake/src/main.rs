use std::process::{Command, exit};
use std::str;
use std::thread;
use std::time::Duration;
use chrono::Local;
use colored::*;

fn main() {
    let mut current_value = 0.000000;
    let mut previous_value = 0.000000;
    let mut amntgained = 0.0;
    let triggertime = 1200 ;
    let threshold = 18.52;
    let bal_output = Command::new("nomic")
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

    // Run the `nomic validators` command
    let output = Command::new("nomic")
        .arg("validators")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");

    // Find the target address and its voting power
    let mut lines = output_str.lines();
    let mut target_voting_power: Option<f64> = None;
    let mut previous_voting_power: Option<f64> = None;
    let mut previous_lines: Vec<&str> = Vec::new();

    while let Some(line) = lines.next() {
        // Keep track of the previous lines
        if previous_lines.len() >= 4 {
            previous_lines.remove(0); // Keep only the last four lines
        }
        previous_lines.push(line);

        // Check for previous voting power before the target address
        if line.contains("nomic1") && !line.contains(target_address) {
            if let Some(vp_line) = lines.next() {
                if let Some(vp_str) = vp_line.split("VOTING POWER: ").nth(1) {
                    previous_voting_power = Some(vp_str.trim().replace(",", "").parse::<f64>().expect("Failed to parse previous voting power"));
                  //  println!("Previous Voting Power Found: {:.10}", previous_voting_power.unwrap());
                }
            }
        }

        // Check for the target address voting power
        if line.contains(target_address) {
            if let Some(vp_line) = lines.next() {
                if let Some(vp_str) = vp_line.split("VOTING POWER: ").nth(1) {
                    target_voting_power = Some(vp_str.trim().replace(",", "").parse::<f64>().expect("Failed to parse target voting power"));
                 //   println!("Target Voting Power Found: {:.10}", target_voting_power.unwrap());
                }
            }
            break; // Stop searching after finding the target address
        }
    }

	if let (Some(vp1), Some(vp2)) = (target_voting_power, previous_voting_power) {
    	let vp1 = vp1 / 1_000_000.0;
    	let vp2 = vp2 / 1_000_000.0;
    	let difference = vp2 - vp1;
    	println!("{}", format! ("The difference in voting power is: {:.4}",difference).yellow().on_black());
   	// println!("Target Voting Power: {:.4}   Previous Voting Power: {:.4}", vp1, vp2);	
	} else {	
	 //   println!("Could not find the required voting powers.");
	}

}


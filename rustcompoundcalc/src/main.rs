use std::process::Command;
use std::env;
use std::process;
use colored::*;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;
use std::str;
use crate::display_help::display_help;

/////MODS///////////////
mod display_help;

fn handle_args(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.contains(&String::from("dels")) {
        delegators()?;
        process::exit(0);
    }

    if args.contains(&String::from("saved")) {
        print_calc_data()?;
        process::exit(0);
    }

    if args.contains(&String::from("git")) {
        display_git_process();
    }

    if args.contains(&String::from("h")) || args.contains(&String::from("help")) {
        display_help();
    }
    if args.contains(&String::from("next")) {
        calculate_voting_power_difference();
        process::exit(0);
    }

    Ok(())
}
fn main() -> io::Result<()> {
    println!("==============================================================================================================================================");
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Error: Not enough arguments provided. You must provide at least four arguments.");
        process::exit(1);
    }
    let mut gains = "";

    if let Err(e) = handle_args(&args) {
        eprintln!("Error handling arguments: {}", e);
        process::exit(1);
    }

    if args.contains(&String::from("gains")) {
        gains = "gains";
    }

    // Parse main input values
    let mut principal: f64 = args[1].parse().expect("Invalid principal amount");
    let fee: f64 = args[2].parse().expect("Invalid fee amount");
    let mut interest_rate: f64 = args[3].parse().expect("Invalid interest rate");
    let commission: f64 = args[4].parse().expect("A commission value is required, put 0 if no commission");

    // Parse `years` argument (allow decimal points)
    let years = if let Some(years_arg) = args.iter().find(|&&ref arg| arg.starts_with("-years")) {
        years_arg.trim_start_matches("-years").parse::<f64>().unwrap_or(1.0)
    } else {
        1.0
    };

    // Parse `terms` argument (allow decimal points)
    let terms = if let Some(terms_arg) = args.iter().find(|&&ref arg| arg.starts_with("-terms")) {
        terms_arg.trim_start_matches("-terms").parse::<f64>().unwrap_or(1.0)
    } else {
        1.0
    };

    interest_rate *= years; // This should already handle decimals correctly
    if commission > 0.0 {
        interest_rate -= interest_rate * commission;
    }

    // Loop for the number of terms (use a loop that runs an integer number of times)
    let term_iterations = terms.round() as usize; // Round to the nearest whole number

    for _ in 0..term_iterations {
        // Initialize variables for the current loop
        let max_freq: usize = (years * 365.0 * 2.0) as usize;
        let untouched = (1.0 + interest_rate) * principal;
        let untouched_claim = interest_rate * principal;

        let mut best_freq = 0;
        let mut best_balance = f64::MIN;
        let mut best_profit = f64::MIN;
        let mut optimal_freq_claim = 0.0;
        let mut optimal_daystoclaim = 0.0;

        // First loop to find the optimal frequency
        for freq in 1..=max_freq {
            let freq_percent = interest_rate / freq as f64;
            let mut future_value = principal;

            for _ in 0..freq {
                future_value += (future_value * freq_percent) - fee;
            }

            let strat_profit = future_value - untouched;
            if strat_profit > best_profit {
                best_profit = strat_profit;
                best_balance = future_value;
                best_freq = freq;
            }
        }

        // Second loop to print with `-found!-` marker at the optimal frequency
        for freq in 1..=max_freq {
            let freq_percent = interest_rate / freq as f64;
            let mut future_value = principal;

            for _ in 0..freq {
                future_value += (future_value * freq_percent) - fee;
            }

            let claim_amount = (interest_rate / freq as f64) * principal;
            let fees = fee * freq as f64;
            let strat_profit = future_value - untouched;

            if gains == "gains" {
                let marker = if freq == best_freq { "-found!-" } else { "" };
                println!(
                    "{} Claiming every {} days yields {}, losing {} to fees, with a net profit of {}, and a balance of {} {}",
                    format!("{:.2}", freq),
                    format!("{:.2}", (years * 365.0) / freq as f64).cyan(),
                    format!("{:.2}", claim_amount).green(),
                    format!("{:.2}", fees).red(),
                    format!("{:.3}", strat_profit).blue(),
                    format!("{:.4}", future_value).yellow(),
                    marker
                );
            }
        }

        // Store the optimal values for use in the final print
        optimal_daystoclaim = 365.0 * years / best_freq as f64;
        optimal_freq_claim = principal * (interest_rate / best_freq as f64);

        println!(
            "The optimal {} claiming frequency for a balance of {} is {} days for a {} year term. \nThis strategy yields {} per claim. With a new balance of {} and a total gain of {} after {} years. \nThis strategy yielded you {} more than not frequently compounding.",
            format!("{:.2}", best_freq).bright_yellow(),
            format!("{:.2}", principal).bright_yellow(),
            format!("{:.4}", optimal_daystoclaim).bright_green(),
            format!("{:.3}", years).bright_green(),
            format!("{:.2}", optimal_freq_claim).bright_green(),
            format!("{:.4}", best_balance).bright_green(),
            format!("{:.2}", best_balance - principal).bright_green(),
            format!("{:.3}", years).bright_green(),
            format!("{:.2}", best_profit).bright_green(),
        );

        println!(
            "If you chose not to compound frequently, you would have only totaled {} with a claim of {}",
            format!("{:.2}", untouched).bright_red(),
            format!("{:.2}", untouched_claim).bright_red(),
        );

        // Update principal for next term iteration
        principal = best_balance;

        // Save the data if the `-save` argument is provided
        if let Some(slot_arg) = args.iter().find(|&&ref arg| arg.starts_with("-save")) {
            if let Ok(slot) = slot_arg.trim_start_matches("-save").parse::<usize>() {
                save_calc_data(slot, principal, optimal_freq_claim, optimal_daystoclaim)?;
            }
        }
    }
    Ok(())
}
////////////END OF MAIN FUNCTION/////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct ExecutionData {
    principal: f64,
    optimal_freq_claim: f64,
    optimal_daystoclaim: f64,
    
}

fn save_calc_data(slot: usize, principal: f64, optimal_freq_claim: f64, optimal_daystoclaim: f64) -> io::Result<()> {
    let file_path = "/home/vboxuser/_coding/datasaves/calc_saves.json";
    let mut executions: HashMap<String, ExecutionData> = if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Use the provided slot for saving
    let key = format!("{}", slot);
    executions.insert(
        key,
        ExecutionData {
            principal,
            optimal_freq_claim,
            optimal_daystoclaim,
        },
    );

    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &executions)?;

    Ok(())
}

fn print_calc_data() -> io::Result<()> {
    let file_path = "/home/vboxuser/_coding/datasaves/calc_saves.json";
    let data = fs::read_to_string(file_path)?;
    let executions: HashMap<String, ExecutionData> = serde_json::from_str(&data)?;

    // Convert HashMap to a Vec of tuples and keep only the last 5 entries
    let mut entries: Vec<(String, ExecutionData)> = executions.into_iter().collect();
    entries.sort_by_key(|(key, _)| key.clone()); // Ensure you sort by key if needed
    entries.truncate(5); // Keep only the most recent 5 entries

    // Print the saved data with the actual slot number
    for (key, value) in entries.iter() {
        println!(
            "Save {}: With a principal of {} claim every {} days with a claimable balance of {} to yield the best results.",
            key,  // Use the slot number as stored in the key
            format!("{}", value.principal).green(),
            format!("{:.2}", value.optimal_daystoclaim).green(),
            format!("{:.2}", value.optimal_freq_claim).green()
        );
    }
    Ok(())
}

fn delegators() -> Result<(), Box<dyn Error>> {
    let file_path = "/home/vboxuser/_coding/datasaves/shares_output.csv";
    // Create a HashMap to store addresses and shares
    let mut address_shares: HashMap<String, f64> = HashMap::new();
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    // Read the CSV file
    for result in rdr.records() {
        let record = result?; // Get the record
        let address = &record[0]; // First column is the address
        let shares: f64 = record[1].parse()?; // Second column is the shares
        // Store in the HashMap
        address_shares.insert(address.to_string(), shares);
    }

    // Example: Access and print addresses and shares
    for (address, shares) in &address_shares {
        println!("Address: {}, Shares: {:.6}", address, shares);

    }
        println!("Usage: <principal> <fee> <max_freq> <interest> <commission> optional: <asset name> <gains>");
    Ok(())
}

fn display_git_process() {  
    println!(" git status \n git add . or git add file name \n git commit -m Your commit message \n git push origin branch-name");
    process::exit(0);
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


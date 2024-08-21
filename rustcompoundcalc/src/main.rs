//use std::process::Command;
use std::env;
use std::process;
use colored::*;
//use std::fs;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;

/////MODS///////////////
//mod nomic_commands;
////////MAIN////////////////////////////////////////////////////
fn main() -> io::Result<()> {
    println!("==============================================================================================================================================");
    let args: Vec<String> = env::args().collect();  
//////////////////DELEGATIONS COMMAND OPTION///////////////////////////////////////    
    if args.contains(&String::from("dels")) {
        if let Err(e) = delegators() {
            eprintln!("Error executing delegators: {}", e);
        }
        process::exit(0); // Exit after running 
    }      
//////////////////////CHECK SAVED DATA COMMAND////////////    
    if args.contains(&String::from("saved")) {
        if let Err(e) = print_execution_data() {
            eprintln!("Error executing prin_execution_data: {}", e);
        }
        process::exit(0); // Exit after running 
    }    
////////////////////DISPLAY GIT PROCESS////////////////  
    if args.contains(&String::from("git")) {
        display_git_process();
    }        
//////////////////HELP COMMAND OPTION///////////////////////////////////////   
    if args.contains(&String::from("h")) || args.contains(&String::from("help")) {
        display_help();
     }
///////////////////////////GAINS COMMAND OPTION//////////////////////////////////////////////// 
let mut gains = "";   
     if args.contains(&String::from("gains")) {
     gains = "gains"
     }
	
    ///////////////////ARGUMENTS////////////////////
    let principal: f64 = args[1].parse().expect("Invalid principal amount");
    let fee: f64 = args[2].parse().expect("Invalid fee amount");
    let max_freq: usize = args[3].parse().expect("Invalid maximum frequency");
    let mut interest_rate: f64 = args[4].parse().expect("Invalid interest rate");
    let commission: f64 = args[5].parse().expect("A commission value is required, put 0 if no commission");       
  ////////////////////BEGINNING OF CALCULATIONS AND VARIABLES/////////////////////////////////////  
      let years = if let Some(years_arg) = args.iter().find(|&&ref arg| arg.starts_with("-years")) {
        years_arg.trim_start_matches("-years").parse::<f64>().unwrap_or(1.0) // Default to 1.0 if parsing fails
    } else {
        1.0 // Default value if the argument is not provided
    };
   // let years: f64 = 20.0;
    interest_rate = interest_rate * years;
    let mut optimal_freq = 0;
    let mut max_profit = f64::MIN;
    let mut optimal_strat_profit = 0.0;
    let mut optimal_freq_balance = 0.0;
    if commission > 0.0 
    {
    	interest_rate = interest_rate - (interest_rate * commission);  
    }
    /////////////UNTOUCHED CLAIM///////////////////////////////        
    let untouched = ( 1.0 + interest_rate) * principal;
    let untouched_claim = interest_rate * principal;      
    //////////////MAIN LOOP///////////////////////////
    for freq in 1..=max_freq {
        let freq_percent = interest_rate / freq as f64;
        let mut future_value_old = principal;
        let mut future_value_new = 0.0;

        for _ in 0..freq {
        let future_value_og = future_value_old;
            future_value_new = future_value_og + ((future_value_old  * freq_percent) -fee);
            future_value_old = future_value_new;
            if gains == "gains"{
 		 // println!("Count:{} OG: {:.2} OLD:{:.2}",freq ,future_value_og, future_value_old);
		}		
        }
        let claim_amount = (interest_rate / freq as f64) * principal;
        let fees = fee * freq as f64 * (1.0 + interest_rate);
        let strat_profit = future_value_new - untouched - fees;
	
        if strat_profit > max_profit {
            max_profit = strat_profit;
            optimal_freq = freq;
            optimal_strat_profit = strat_profit;
            optimal_freq_balance = future_value_new; // Store the future_value_new at the optimal frequency
        }
	
        if gains == "gains" {
            println!(
                "Claiming every {} days yields {}, losing {} to fees, with a net profit of {}.",
                format!("{}", freq).cyan(),
                format!("{:.2}", claim_amount).green(),
                format!("{:.2}", fees).red(),
                format!("{:.2}", strat_profit).blue(),
            );
        }
    }

    ////END OF LOOP///////////////////////
    println!("\n");
    let optimal_daystoclaim = 365.0 * years / optimal_freq as f64;
    let optimal_freq_claim = principal * (interest_rate / optimal_freq as f64);
    println!( "The optimal claiming frequency for a blance of {} is {} days for a {:.2} year term. \nThis strategy yields {} per claim. With a new balance of {} and a total gain of {:.2} after {} years. \nThis strategy yielded you {} more than not frequenctly compounding.",
	format!("{:.2}",principal).bright_yellow(),
        format!("{:.2}",optimal_daystoclaim).bright_green(),
        format!("{}",years).bright_green(),	        
        format!("{:.2}",optimal_freq_claim).bright_green(),
        format!("{:.2}", optimal_freq_balance).bright_green(),
        format!("{:.2}", optimal_freq_balance - principal).bright_green(),
        format!("{}",years).bright_green(),                
        format!("{:.2}",optimal_strat_profit).bright_green(),
    ); 
    
    println!("If you chose not to compound frequently, you would have only totaled {} with a claim of {}",
    	format!("{:.2}", untouched).bright_red(),
    	format!("{:.2}", untouched_claim).bright_red(),
    );
    println!("\n");    
////////////////SAVING DATA/////////////////////////////////////
 if let Some(slot_arg) = args.iter().find(|&&ref arg| arg.starts_with("-save")) {
        if let Ok(slot) = slot_arg.trim_start_matches("-save").parse::<usize>() {
            save_execution_data(slot, principal, optimal_freq_claim, optimal_daystoclaim).expect("Failed to save execution data.");
        }
    }

    // Example of how to print the execution data
    print_execution_data().expect("Failed to print execution data.");
        Ok(())
}
////////////END OF MAIN FUNCTION/////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct ExecutionData {
    principal: f64,
    optimal_freq_claim: f64,
    optimal_daystoclaim: f64,
    
}

fn save_execution_data(slot: usize, principal: f64, optimal_freq_claim: f64, optimal_daystoclaim: f64) -> io::Result<()> {
    let file_path = "execution_data.json";
    let mut executions: HashMap<String, ExecutionData> = if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Use the provided slot for saving
    let key = format!("principal{}", slot);
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

fn print_execution_data() -> io::Result<()> {
    let file_path = "execution_data.json";
    let data = fs::read_to_string(file_path)?;
    let executions: HashMap<String, ExecutionData> = serde_json::from_str(&data)?;

    // Convert HashMap to a Vec of tuples and keep only the last 5 entries
    let mut entries: Vec<(String, ExecutionData)> = executions.into_iter().collect();
    entries.sort_by_key(|(key, _)| key.clone()); // Ensure you sort by key if needed
    entries.truncate(5); // Keep only the most recent 5 entries

    // Print the saved data with the actual slot number
    for (key, value) in entries.iter() {
        println!(
            "Save {}: With a principal of {} you should compound when you have claimable balance of {} which is every {} days.",
            key,  // Use the slot number as stored in the key
            format!("{}", value.principal).green(),
            format!("{:.2}", value.optimal_freq_claim).green(),
            format!("{:.2}", value.optimal_daystoclaim).green()
        );
    }

    Ok(())
}


fn delegators() -> Result<(), Box<dyn Error>> {
    let file_path = "/home/vboxuser/Desktop/shares_output.csv";
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
// Function to display the help section
fn display_help() {
    println!();
    println!("This program calculates the future value of an investment over various frequencies, accounting for interest and fees.");
    println!();
    println!("Arguments:");
    println!("  principal   The initial amount of money invested or loaned (e.g., 1000).");
    println!("  fee         The fee per period (e.g., 1.5).");
    println!("  max_freq    The maximum number of periods to test (e.g., 365).");
    println!("  interest    The annual interest rate as a decimal (e.g., 0.05 for 5%).");
    println!("  commission  The commission charged as decimal (e.g, 0.05 for 5%");
    println!("  -save       To save a particular run if you wish to keep the data handy");    
    println!("  saved       To access previous calculations");
    println!("  git         For git commit process       ");
    println!("  <gains>       To read out each compound calculations");
    println!(" ");    
    println!("Usage:<principal> <fee> <max_freq> <interest> <commission> optional: <-save> <gains> <git> <saved> <\"asset name\">");
    println!("Example:  1000 0.01 365 0.26 0.05 <options>");
    process::exit(0);
}
fn display_git_process() {  
    println!(" git status \n git add . or git add file name \n git commit -m Your commit message \n git push origin branch-name");
    process::exit(0);
}


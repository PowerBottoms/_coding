use std::env;
use std::process;
//use std::process::Command;
use colored::*;
mod nomic_commands;

// Function to display the help section
fn display_help() {
    println!("Usage: <program> <principal> <fee> <max_freq> <interest> optional: <gains>");
    println!();
    println!("This program calculates the future value of an investment over various frequencies, accounting for interest and fees.");
    println!();
    println!("Arguments:");
    println!("  principal   The initial amount of money invested or loaned (e.g., 1000).");
    println!("  fee         The fee per period (e.g., 1.5).");
    println!("  max_freq    The maximum number of periods to test (e.g., 365).");
    println!("  interest    The annual interest rate as a decimal (e.g., 0.05 for 5%).");
    println!("  commission  The commission charged as decimal (e.g, 0.05 for 5%");
    println!();
    println!("Options:");
    println!("  -h, --help  Display this help message and exit.");
    println!();
    println!("Example:");
    println!("  <program> 1000 1.5 365 0.05 gains");
    process::exit(0);
}

fn main() {
    println!("==============================================================================================================================================");
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: <program> <principal> <fee> <max_freq> <interest rate> OPTIONAL: <asset name>, <gains>");
        eprintln!("Try '<program> --help' for more information.");
        process::exit(1);
    }

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        display_help();
    }

    ///////////////////ARGUMENTS////////////////////
    let principal: f64 = args[1].parse().expect("Invalid principal amount");
    let fee: f64 = args[2].parse().expect("Invalid fee amount");
    let max_freq: usize = args[3].parse().expect("Invalid maximum frequency");
    let mut interest_rate: f64 = args[4].parse().expect("Invalid interest rate");
    let commission: f64 = args[5].parse().expect("A commission value is required, put 0 if no commission");       
    /////////////OPTIONAL ARGUMENTS/////////////
    let asset_name = if args.len() > 6 { &args[6] } else {
        "" // Default value if the argument is not provided
    }; 
    let gains = args.get(7).map(|s| s.as_str()).unwrap_or("Optional: shows gains accumulating");

    let mut optimal_freq = 0;
    let mut max_profit = f64::MIN;
    
    let mut optimal_strat_profit = 0.0;
    let mut optimal_freq_balance = 0.0;
    //////////////TRYING TO CALCULATE COMMISSION////////
    if commission > 0.0 
    {
    interest_rate = interest_rate - (interest_rate * commission);  
    }
    /////////////UNTOUCHED CLAIM///////////////////////////////        
    let untouched = ( 1.0 + interest_rate) * principal;
    let untouched_claim = interest_rate * principal;      
    //////////////MAIN LOOP///////////////////////////
    for freq in 1..=max_freq {

        let daily_percent = interest_rate / freq as f64;

        let mut future_value_old = principal;
        let mut future_value_new = 0.0;

        for _ in 0..freq {
            future_value_new = future_value_old + (future_value_old - fee) * daily_percent;
            future_value_old = future_value_new;
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
    let freq_time = 365.0 / optimal_freq as f64;
    let optimal_freq_claim = principal * (interest_rate / optimal_freq as f64);
    println!( "The optimal claiming frequency for a blance of {} is {} days. \nThis strategy yields {} per claim. With a new balance of {} and a total yearly gain of {}. \nThis strategy yielded you {} more {} than not frequenctly compounding.",
	format!("{:.2}",principal).bright_yellow(),
        format!("{:.2}",freq_time).bright_green(),
        format!("{:.2}",optimal_freq_claim).bright_green(),
        format!("{:.2}", optimal_freq_balance).bright_green(),
        format!("{:.2}", optimal_freq_balance - principal).bright_green(),        
        format!("{:.2}",optimal_strat_profit).bright_green(),
        format!("{}",asset_name).bright_blue(),
    ); 
  //  println!("With a potential new balance of {} and a gain of {}",  );
    println!("If you chose not to compound frequently, you would have only totaled {} with a claim of {}",
    	format!("{:.2}", untouched).bright_red(),
    	format!("{:.2}", untouched_claim).bright_red(),
    );
    println!("\n");    
    println!(" git status \n git add . or git add file name \n git commit -m Your commit message \n git push origin branch-name");
    println!("\n");
//////////NOMIC COMMANDS FROM OTHER SCRIPTS//////////////////////////    
    match nomic_commands::get_nomic_balance() {
        Ok(balance) => println!("Nomic Balance: {}", balance),
        Err(e) => eprintln!("Error: {}", e),
    }

}


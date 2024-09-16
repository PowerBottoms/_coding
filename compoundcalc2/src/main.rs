use std::env;
use std::io;
use std::process;
use colored::*;

fn print_frequency_results(freq: usize, total_minutes: f64, interest_rate: f64, principal: f64, fee: f64, future_value: f64, untouched: f64, strat_profit: f64) {
    println!(
        "{} Claiming every {} minutes yields {}, losing {} to fees, with a net profit of {}, and a balance of {}",
        format!("{:.2}", freq),
        format!("{:.4}", total_minutes / freq as f64).cyan(),
        format!("{:.2}", (interest_rate / freq as f64) * principal).green(),
        format!("{:.2}", fee * freq as f64).red(),
        format!("{:.3}", strat_profit).blue(),
        format!("{:.4}", future_value).yellow(),
    );
}

fn print_optimal_frequency(best_freq: usize, principal: f64, optimal_daystoclaim: f64, years: f64, optimal_freq_claim: f64, best_balance: f64, best_profit: f64) {
    println!(
        "The optimal {} claiming frequency for a balance of {} is {} minutes for a {} year term. \nThis strategy yields {} per claim. With a new balance of {} and a total gain of {} after {} years. \nThis strategy yielded you {} more than not frequently compounding.",
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
}

fn print_no_compound_results(untouched: f64, untouched_claim: f64) {
    println!(
        "If you chose not to compound frequently, you would have only totaled {} with a claim of {}",
        format!("{:.2}", untouched).bright_red(),
        format!("{:.2}", untouched_claim).bright_red(),
    );
}

fn main() -> io::Result<()> {
    println!("==============================================================================================================================================");
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Error: Not enough arguments provided. You must provide at least four arguments.");
        process::exit(1);
    }

    // Parse main input values
    let mut principal: f64 = args[1].parse().expect("Invalid principal amount");
    let fee: f64 = args[2].parse().expect("Invalid fee amount");
    let mut interest_rate: f64 = args[3].parse().expect("Invalid interest rate");
    let commission: f64 = args[4].parse().expect("A commission value is required, put 0 if no commission");

    // Parse `years` argument (allow decimal points)
    let years = if let Some(years_arg) = args.iter().find(|&&ref arg| arg.starts_with("-y")) {
        years_arg.trim_start_matches("-y").parse::<f64>().unwrap_or(1.0)
    } else {
        1.0
    };

    // Calculate total time in minutes for the specified years
    let total_minutes = years * 365.0 * 24.0 * 60.0; // Convert years to minutes

    interest_rate *= years; // Adjust interest rate based on the number of years
    if commission > 0.0 {
        interest_rate -= interest_rate * commission;
    }

    // Initialize variables for the calculation
    let untouched = (1.0 + interest_rate) * principal;
    let untouched_claim = interest_rate * principal;

    let mut best_freq = 0;
    let mut best_balance = f64::MIN;
    let mut best_profit = f64::MIN;
    let mut optimal_freq_claim = 0.0;
    let mut optimal_daystoclaim = 0.0;
    let start_principal = principal;

    // Change the loop range and step
    let step = 10.0; // This is how much the frequency increments, change for performance tuning
    for freq in (1..=(total_minutes as f64 / step) as usize).rev() {
        let freq_in_minutes = freq as f64 * step; // Calculate frequency in minutes
        let freq_percent = interest_rate / total_minutes * freq_in_minutes; // Interest rate per period

        let mut future_value = start_principal;

        // Simulate compounding based on frequency
        let num_claims = (total_minutes / freq_in_minutes).floor() as usize;
        for _ in 0..num_claims {
            let claim_amount = (future_value * freq_percent).max(0.0);
            future_value += claim_amount - fee; // Apply compounding formula
        }

        let strat_profit = future_value - start_principal;

        // Print results for each frequency
        print_frequency_results(freq, total_minutes, interest_rate, principal, fee, future_value, untouched, strat_profit);

        // Update best balance and frequency if profit increases
        if strat_profit > best_profit {
            best_profit = strat_profit;
            best_balance = future_value;
            best_freq = freq;
        }
    }

    // Store the optimal values for use in the final print
    optimal_daystoclaim = total_minutes / best_freq as f64;
    optimal_freq_claim = principal * (interest_rate / best_freq as f64);

    print_optimal_frequency(best_freq, principal, optimal_daystoclaim, years, optimal_freq_claim, best_balance, best_profit);
    print_no_compound_results(untouched, untouched_claim);

    Ok(())
}


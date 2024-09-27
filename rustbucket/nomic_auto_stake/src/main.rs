use std::env;
use std::process::{Command, exit};
use std::str;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{Local, DateTime, Utc, Duration as ChronoDuration, Timelike, Datelike, Weekday};
use colored::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation; 
use regex::Regex;



fn main() {

    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Set the default triggertime to 600 if no argument is provided
    let triggertime: u64 = if args.len() > 1 {
        args[1].parse().unwrap_or(600)
    } else {
        600 // Default value
    };
    let num_comparisons = 31; 
    let mut current_value = 0.000000;
    let mut previous_value = 0.000000;
    let mut amntgained = 0.0;
    let threshold = 20.64;
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
        println!("Warning: No liquid amount found. Continuing without liquid balance...");
        // Provide a default value or continue with the rest of the script
        "liquid=0" // A default value if needed
    }
};
/*
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
        }; */

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
                amntgained).truecolor(218,165,32).on_truecolor(0,0,0)
            );
            let _time_to_claim = calc_tt_claim(formatted_claimable, threshold, amntgained, triggertime);
            println!("{}",format! ("Time to claim {}",_time_to_claim).truecolor(218,165,32).on_truecolor(0,0,0));
            
            // Calculate voting power spreads and get vp1 if available
            if let Some(vp1) = calculate_vp_spreads(num_comparisons) {
                // Calculate APR with vp1
                let apr = calculate_apr(vp1, triggertime, amntgained);
                println!("{}", format! ("Current APR:{:.4}%", apr).truecolor(218,165,32).on_truecolor(0,0,0));
            } else {
                println!("Failed to calculate APR due to missing voting power.");
            }

            println!("{}", "--------------------------------------------------------------------------------------".truecolor(50,50,50).on_truecolor(0, 0, 0));
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
        println!("You have claimed and staked {}", formatted_delegated.to_string().truecolor(218,165,32).on_truecolor(0,0,0));
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
    
pub struct CustomColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl From<CustomColor> for Color {
    fn from(color: CustomColor) -> Self {
        Color::TrueColor { r: color.r, g: color.g, b: color.b }
    }
}

fn calculate_vp_spreads(num_comparisons: usize) -> Option<f64> {
        let now = Utc::now();
    let target_address = "nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u";
    let best_spreads = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/best_vp_spreads.txt"; // File to save smallest spreads
    let worst_spreads_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/worst_voting_power_spreads.txt"; // File to save worst spreads
    let current_spreads_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/current_vp_spreads.txt"; // File to save current spreads
    let weekly_spreads_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/weekly_vp_spreads.txt"; // File to save current spreads  
    let weekly_save_count_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/weekly_save_count.txt";
     
    // Load weekly_save_count from file
    let mut weekly_save_count = if Path::new(weekly_save_count_path).exists() {
        let file = File::open(weekly_save_count_path).expect("Unable to open weekly save count file");
        let reader = io::BufReader::new(file);
        reader.lines().next().unwrap().unwrap().parse::<i32>().unwrap()
    } else {
        0
    };      
    if now.weekday() != Weekday::Thu { weekly_save_count = 0; println!("Week save count set to 0"); } 
   
  //  let last_saved_path = "/home/vboxuser/_coding/rustbucket/nomic_auto_stake/src/last_weekly_save.txt";
    let emoji_regex = Regex::new(r"[\p{Emoji}]").unwrap(); // Regex to match emojis
    /////////////////////Custom Colors///////////////////////
    let custom_green = CustomColor { r: 110, g: 198, b: 110 };
    let custom_red = CustomColor { r: 205, g: 92, b: 92 };
    let custom_navy = CustomColor { r: 0, g: 0, b: 176 };
    let custom_sea_green = CustomColor { r: 60, g: 179, b: 113 };
    let olive_green = CustomColor { r: 85, g: 107, b: 47 };
    let goldenrod = CustomColor { r: 218, g: 165, b: 32 }; 
    // Convert to Color for use with colored crate
    let c_navy = Color::from(custom_navy);
    let c_red = Color::from(custom_red);
    let c_green = Color::from(custom_green);
    let c_sea_green = Color::from(custom_sea_green);
    let c_olive_green = Color::from(olive_green);
    let c_goldenrod = Color::from(goldenrod);
    
    // Load smallest and largest spreads from the file
    let mut smallest_spreads = vec![f64::INFINITY; num_comparisons];
    let mut largest_spreads = vec![f64::NEG_INFINITY; num_comparisons]; // Store largest spreads
    let mut weekly_spreads = vec![f64::NEG_INFINITY; num_comparisons];
    let mut current_spreads = Vec::new();
    
    

    // Load best spreads from the file
    if Path::new(best_spreads).exists() {
        let file = File::open(best_spreads).expect("Unable to open best spreads file");
        let reader = io::BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            if i < num_comparisons {
                if let Ok(value) = line {
                    if let Ok(spread) = value.trim().parse::<f64>() {
                        smallest_spreads[i] = spread;
                    }
                }
            }
        }
    }

    // Load worst spreads from the file if it exists
    if Path::new(worst_spreads_path).exists() {
        let file = File::open(worst_spreads_path).expect("Unable to open worst spreads file");
        let reader = io::BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            if i < num_comparisons {
                if let Ok(value) = line {
                    if let Ok(spread) = value.trim().parse::<f64>() {
                        largest_spreads[i] = spread; // Load largest spreads
                    }
                }
            }
        }
    }
    // Load weekly spreads if the exist
    if Path::new(weekly_spreads_path).exists() {
        let file = File::open(weekly_spreads_path).expect("Unable to open worst spreads file");
        let reader = io::BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            if i < num_comparisons {
                if let Ok(value) = line {
                    if let Ok(spread) = value.trim().parse::<f64>() {
                        weekly_spreads[i] = spread; // Load weekly spreads
                    }
                }
            }
        }
    }	
    // Load current spreads if they exist
    if Path::new(current_spreads_path).exists() {
        let file = File::open(current_spreads_path).expect("Unable to open current spreads file");
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(value) = line {
                if let Ok(spread) = value.trim().parse::<f64>() {
                    current_spreads.push(spread);
                }
            }
        }
    }


    // Run the nomic validators command
    let output = Command::new("nomic")
        .arg("validators")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");

    // Find the target address, voting power, and moniker
    let lines = output_str.lines().collect::<Vec<_>>();
    let mut target_voting_power: Option<f64> = None;
    let mut voting_powers: Vec<f64> = Vec::new();
    let mut monikers: Vec<String> = Vec::new();

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

            if let Some(moniker_line) = lines.get(i + 2) {
                let moniker = moniker_line.replace("MONIKER: ", "");
                monikers.push(moniker.trim().to_string());
            }

            break;
        }
    }

    // If the target address is found, look back for previous addresses, voting powers, and monikers
    if let Some(index) = target_index {
        let mut look_back_count = 0;
        let mut i = index as isize - 1;

        while look_back_count < num_comparisons && i >= 0 {
            let line = lines[i as usize];
            if line.contains("nomic1") {
                if let Some(vp_line) = lines.get(i as usize + 1) {
                    if let Some(vp_str) = vp_line.split("VOTING POWER: ").nth(1) {
                        let voting_power = vp_str.trim().replace(",", "").parse::<f64>().expect("Failed to parse voting power");
                        voting_powers.push(voting_power);
                    }
                }

                if let Some(moniker_line) = lines.get(i as usize + 2) {
                    let moniker = moniker_line.replace("MONIKER: ", "");
                    monikers.push(moniker.trim().to_string());
                }

                look_back_count += 1;
            }
            i -= 1;
        }
    }

    // If we have the target voting power and enough previous voting powers, calculate the spreads
    if let Some(vp1) = target_voting_power {
        let vp1 = vp1;

        let voting_powers = voting_powers.into_iter().take(num_comparisons).collect::<Vec<_>>();
        let monikers = monikers.into_iter().take(num_comparisons).collect::<Vec<_>>();

        let vp_spreads = voting_powers.iter().map(|&vp| (vp - vp1).abs()).collect::<Vec<_>>();


 if now.weekday() == Weekday::Thu && weekly_save_count == 0 {
    // Load existing weekly spreads using load_weekly_spreads function
    let mut weekly_spreads = load_weekly_spreads(weekly_spreads_path);

    // Add current week's spreads
    weekly_spreads.push(vp_spreads.clone());

    // Keep only the last 10 weeks
    if weekly_spreads.len() > 10 {
        weekly_spreads.drain(..weekly_spreads.len() - 10);
    }

    // Print the updated weekly spreads
    println!("\nUpdated Weekly Spreads (including current week):");
    for (week, spreads) in weekly_spreads.iter().enumerate() {
        println!("Week {}: {:?}", week + 1, spreads);
    }

    // Save all weekly spreads
    let mut weekly_file = File::create(weekly_spreads_path).expect("Unable to create weekly spreads file");
    for week in &weekly_spreads {
        let spreads_str = week.iter().map(|&s| format!("{:.2}", s)).collect::<Vec<String>>().join(",");
        writeln!(weekly_file, "{}", spreads_str).expect("Unable to write to weekly spreads file");
    }

    weekly_save_count = 1;
    println!("Success! Weekly spreads have been saved. Weekly save count is {}", weekly_save_count);

    // Save the updated weekly_save_count
    let mut count_file = File::create(weekly_save_count_path).expect("Unable to create weekly save count file");
    writeln!(count_file, "{}", weekly_save_count).expect("Unable to write to weekly save count file");
} else {
    println!("Weekly save count is {}", weekly_save_count);
}

println!(
    "{}",
    "      |   Current  | |    Best    | |    Worst   |   Change   |        Moniker       |   Est. Time to Zero    | ".truecolor(90,90,90).on_truecolor(0,0,0)
);
for (i, spread) in vp_spreads.iter().enumerate() {
    if let Some(moniker) = monikers.get(i + 1) {
        let formatted_spread = spread / 1_000_000.0;
        let formatted_best = smallest_spreads[i] / 1_000_000.0;
        let formatted_worst = largest_spreads[i] / 1_000_000.0;
        let spread_change = (formatted_worst - formatted_spread).abs();

        // Remove emojis from the moniker
        let clean_moniker = emoji_regex.replace_all(moniker, "").to_string();

        // Truncate the moniker to a maximum of 17 characters
        let truncated_moniker = if clean_moniker.len() > 17 {
            format!("{}", &clean_moniker[..17])
        } else {
            clean_moniker // Keep the original if it's short enough
        };

        // Update smallest and largest spreads
        if formatted_spread < formatted_best {
            smallest_spreads[i] = *spread; // Update best spreads
        }
        if formatted_spread > formatted_worst {
            largest_spreads[i] = *spread; // Update worst spreads
        }

        let current_color = if formatted_spread < formatted_worst && formatted_spread > formatted_best {
            c_olive_green
        } else if formatted_spread == formatted_best { 
            c_green  
        } else { 
            c_red
        };
        let spread_color = if formatted_spread == formatted_best {  
            c_olive_green
        } else {  
            Color::TrueColor { r: 205, g: 92, b: 92 }  // Custom red color
        };

        let worst_color = if formatted_spread < formatted_worst {
            c_olive_green
        } else {
            Color::TrueColor { r: 205, g: 92, b: 92 }
        };

        let change_color = if spread_change > 0.0 { 
            c_olive_green
        } else { 
            Color::TrueColor { r: 205, g: 92, b: 92 }
        };

        // Assuming `weekly_spreads` is a vector of vectors, where each inner vector is a week's data for all monikers.
// Assuming `est_time_to_zero` is defined as a vector of strings, where each index corresponds to the respective moniker.
let mut est_time_to_zero = Vec::new();

    if weekly_spreads.len() >= 2 {
        let weekly_spreads = load_weekly_spreads(weekly_spreads_path); // Ensure weekly_spreads are loaded
        for moniker_index in 0..weekly_spreads[0].len() { // Iterate over the number of monikers
            let mut moniker_spread = Vec::new();
            for week_data in &weekly_spreads {
                moniker_spread.push(week_data[moniker_index]);
            }
            
            let first_week = moniker_spread[0];
            let last_week = moniker_spread[moniker_spread.len() - 1];
            let change_over_period = first_week - last_week;
            let weeks = moniker_spread.len() as f64;

            let mut days_to_zero = if change_over_period != 0.0 {
                last_week / (change_over_period / weeks) * 7.0 // converting weeks to days
            } else {
                f64::INFINITY // No change; no time to zero
            };

// Assume these constants are defined somewhere for conversion
// Constants for conversion
const DAYS_IN_MONTH: f64 = 30.44; // Average days in a month
const DAYS_IN_WEEK: f64 = 7.0;    // Days in a week

// Format time-to-zero value for this specific moniker
if days_to_zero.is_finite() && days_to_zero > 0.0 {
    // Calculate months, weeks, and remaining days
    let months = (days_to_zero / DAYS_IN_MONTH).floor();
    let remaining_days_after_months = days_to_zero % DAYS_IN_MONTH;
    let weeks = (remaining_days_after_months / DAYS_IN_WEEK).floor();
    let days = remaining_days_after_months % DAYS_IN_WEEK;
    let spacer = "";
    // Construct the formatted string
    let mut time_to_zero_parts = Vec::new();

    if months >= 0.0 {
        time_to_zero_parts.push(format!("{:>5.0}M", months).color(c_olive_green).to_string());  // Show months as integers
    }
    if weeks >= 0.0 {
        time_to_zero_parts.push(format!("{:.0}W", weeks).color(c_olive_green).to_string());  // Show weeks as integers
    }
    if days >= 0.0 {
        time_to_zero_parts.push(format!("{:.2}D", days).color(c_olive_green).to_string());   // Show days with 2 decimals
        time_to_zero_parts.push(format!("{:<5}", spacer).color(c_olive_green).to_string());   // Show days with 2 decimals
    }

    // Join the parts into a single string
    let formatted_time_to_zero = time_to_zero_parts.join(" ");

    // Ensure the string is always 20 characters wide, right-aligned
    est_time_to_zero.push(format!("{:<20}", formatted_time_to_zero).color(c_olive_green).to_string());  // Color the entire string green
} else if last_week == 0.0 {
    est_time_to_zero.push(format!("{:>20}", "0 days".color(c_olive_green)).to_string());  // Color "0 days" in green
} else {
    est_time_to_zero.push(format!("{:<21}", "    Losing out".color(c_red)).to_string());      // Color "Losing out" in red
}



        }
    }


// Use the specific estimated time for the current moniker
let current_est_time_to_zero = if i < est_time_to_zero.len() {
    &est_time_to_zero[i]
} else {
    "N/A" // Default value if not available
};

// Adjust the print statement for the current moniker
println!( 
    "{:>5}{}{:>10}{}{:>10}{}{:>10}{}{:>10}{}{:^20}{}{}{}",
    format!("({}) ", i + 1).truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{}"," | ").truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{:.2} ", formatted_spread).color(current_color).on_truecolor(0,0,0),
    format!("{}"," | | ").truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{:.2} ", formatted_best).color(spread_color).on_truecolor(0,0,0),
    format!("{}"," | | ").truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{:.2} ", formatted_worst).color(worst_color).on_truecolor(0,0,0),
    format!("{}"," | ").truecolor(90,90,90).on_truecolor(0,0,0), 
    format!("{:.2} ", spread_change).color(change_color).on_truecolor(0,0,0),
    format!("{}"," | ").truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{} ", truncated_moniker).truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{}"," | ").truecolor(90,90,90).on_truecolor(0,0,0),
    format!("{} ", current_est_time_to_zero).truecolor(90,90,90).on_truecolor(0,0,0), // Use the specific time for this moniker
    format!("{}"," | ").truecolor(90,90,90).on_truecolor(0,0,0)
);

    }
}

        // Save the smallest spreads to file
        let mut best_file = File::create(best_spreads).expect("Unable to create best spreads file");
        for &spread in &smallest_spreads {
            writeln!(best_file, "{:.2}", spread).expect("Unable to write to best spreads file");
        }

        // Save the largest spreads to file
        let mut worst_file = File::create(worst_spreads_path).expect("Unable to create worst spreads file");
        for &spread in &largest_spreads {
            writeln!(worst_file, "{:.2}", spread).expect("Unable to write to worst spreads file");
        }

        // Save current spreads to file
        let mut current_file = File::create(current_spreads_path).expect("Unable to create current spreads file");
        for &spread in &vp_spreads {
            writeln!(current_file, "{:.2}", spread).expect("Unable to write to current spreads file");
        }
        return Some(vp1); // Return the target voting power
    }
    
    None // Return None if target voting power is not found
}

fn load_weekly_spreads(weekly_spreads_path: &str) -> Vec<Vec<f64>> {
    let mut weekly_spreads: Vec<Vec<f64>> = Vec::new();
    if Path::new(weekly_spreads_path).exists() {
        let file = File::open(weekly_spreads_path).expect("Unable to open weekly spreads file");
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(spread_str) = line {
                let spreads: Vec<f64> = spread_str.split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect();
                weekly_spreads.push(spreads);
            }
        }
    }
    weekly_spreads
}

fn calculate_apr(vp1: f64, triggertime: u64, amntgained: f64) -> f64 {
    // Step 1: Get the current staked amount
    let output = Command::new("nomic")
        .arg("delegations")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");

    // Find the line containing the target address
    let target_line = output_str.lines().find(|line| line.contains("nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u"));
    
    if let Some(line) = target_line {
        // Extract the staked amount
        let staked_str = line.split("staked=")
            .nth(1)
            .unwrap_or("0")
            .split_whitespace()
            .next()
            .unwrap_or("0")
            .replace(",", "");
        
        let staked_amount: f64 = staked_str.parse().expect("Failed to parse staked amount");
	
        // Step 2: Calculate x
       // let x = (vp1 - staked_amount) * 0.04 + staked_amount;
	let x = vp1;
        // Step 3: Calculate how many times triggertime goes into 365 days
        let seconds_in_a_year = 365 * 24 * 60 * 60;
        let intervals_per_year = seconds_in_a_year as f64 / triggertime as f64;

        // Step 4: Calculate APR
        let apr = ((amntgained * 1_000_000.0) / x) * intervals_per_year;

        // Return APR as a percentage
        apr * 1.0
    } else {
        println!("Could not find the target address in delegations.");
        0.0 // Return 0 if we can't calculate APR
    }
}

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::collections::HashSet;
use rodio::{Decoder, OutputStream, Sink}; // Import necessary items from rodio

const INTEREST_RATE: f64 = 0.26;
const FEE: f64 = 0.01; // Assume some fee
const COMMISSION: f64 = 0.05; // 5% commission on each compound amount
const UPDATE_INTERVAL: u64 = 60; // Update every 60 seconds
const SPEED: u64 = 100; // Speed set to 1 for real-time

fn calculate_optimal_frequency(principal: f64) -> (u64, Duration) {
    let mut max_profit = 0.0;
    let mut optimal_freq = 1;
    let max_freq = 365; // Example max frequency, one compound per day

    for freq in 1..=max_freq {
        let freq_percent = INTEREST_RATE / freq as f64;
        let mut future_value_old = principal;
        let mut future_value_new = 0.0;

        for _ in 0..freq {
            let compound_amount = future_value_old * freq_percent;
            let after_commission = compound_amount * (1.0 - COMMISSION);
            future_value_new = future_value_old + (after_commission - FEE);
            future_value_old = future_value_new;
        }

        let fees = FEE * freq as f64 * (1.0 + INTEREST_RATE);
        let strat_profit = future_value_new - principal - fees;

        if strat_profit > max_profit {
            max_profit = strat_profit;
            optimal_freq = freq;
        }
    }

    let optimal_duration = Duration::from_secs(365 * 24 * 60 * 60 / optimal_freq as u64); // Convert optimal frequency to a duration
    (optimal_freq, optimal_duration)
}

fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs();
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!(
        "{}d {:02}h {:02}m {:02}s",
        days, hours, minutes, seconds
    )
}

fn print_progress_bar(percentage: f64) -> String {
    let bar_length = 50; // The total length of the progress bar
    let filled_length = (percentage * bar_length as f64 / 100.0).round() as usize;

    let mut bar = String::new();
    bar.push('[');
    for i in 0..bar_length {
        if i < filled_length {
            bar.push('=');
        } else {
            bar.push(' ');
        }
    }
    bar.push(']');
    format!(" {}% {}", percentage.round(), bar)
}

fn print_update_progress(seconds_passed: u64) {
    let bar_length = 60; // The total length of the progress bar for the update
    let progress = (seconds_passed % bar_length) as u64; // Change this to u64

    let mut bar = String::new();
    bar.push('[');
    for i in 0..bar_length {
        if i < progress {
            bar.push('-');
        } else {
            bar.push(' ');
        }
    }
    print!(" Update in: {}s {}\r", bar_length as u64 - progress, bar);
    io::stdout().flush().unwrap(); // Ensure the progress bar updates on the same line
}

fn clear_terminal() {
    // Clear the terminal screen (this works on most Unix-based systems and Windows)
    print!("\x1B[2J\x1B[1;1H");
}

// Function to play sound
fn play_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = std::fs::File::open("/home/vboxuser/_coding/rustcompoundcalc/assets/alert.wav").unwrap(); // Make sure the path is correct
    let source = Decoder::new_wav(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end(); // Wait for the sound to finish
}

fn main() {
    let balances = Arc::new(Mutex::new([1111711.0, 5647.0, 948.0, 498.47]));
    let start_times = Arc::new(Mutex::new([Instant::now(); 4]));

    let balances_clone = Arc::clone(&balances);
    let start_times_clone = Arc::clone(&start_times);

    // Thread for user input
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let mut balances = balances_clone.lock().unwrap();
            let mut start_times = start_times_clone.lock().unwrap();

            if input.trim().starts_with("-claim") {
                if let Ok(index) = input.trim()[6..].parse::<usize>() {
                    if index > 0 && index <= balances.len() {
                        let index = index - 1;
                        let (optimal_freq, _time_to_compound) = calculate_optimal_frequency(balances[index]);
                        let claim_amount = balances[index] * (INTEREST_RATE / optimal_freq as f64) * (1.0 - COMMISSION);
                        balances[index] += claim_amount;
                        start_times[index] = Instant::now(); // Reset the timer for this balance
                    }
                }
            }
        }
    });

    loop {
        clear_terminal();

        let balances = balances.lock().unwrap();
        let start_times = start_times.lock().unwrap();

        for (i, &balance) in balances.iter().enumerate() {
            let (optimal_freq, time_to_compound) = calculate_optimal_frequency(balance);
            let adjusted_elapsed = start_times[i].elapsed().as_secs() * SPEED;
            let remaining_time = time_to_compound.saturating_sub(Duration::from_secs(adjusted_elapsed));
            let percentage = 100.0 * (remaining_time.as_secs_f64() / time_to_compound.as_secs_f64());
            let progress_bar = print_progress_bar(percentage);

            println!(
                "Balance {}: ${:.2} | Optimal Frequency: {} times/year | Time remaining: {} | {}",
                i + 1,
                balance,
                optimal_freq,
                format_duration(remaining_time),
                progress_bar
            );

            // Check if the time to compound has reached
            if remaining_time.is_zero() {
                play_sound(); // Play sound when time is up
            }
        }

        for sec in 0..UPDATE_INTERVAL {
            print_update_progress(sec);
            thread::sleep(Duration::from_secs(1 / SPEED)); // Adjusted sleep duration
        }
    }
}


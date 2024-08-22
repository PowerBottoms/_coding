use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use rodio::{Decoder, OutputStream, Sink}; // Import necessary items from rodio

const INTEREST_RATE: f64 = 0.26;
const BALANCES: [f64; 4] = [1111711.0, 5647.0, 948.0, 498.47];
const UPDATE_INTERVAL: u64 = 60; // Update every 60 seconds
const SPEED: u64 = 100000; // Speed multiplier for testing

fn optimal_compounding_time(balance: f64) -> Duration {
    let optimal_days = 365.0 / (INTEREST_RATE * balance.log(2.0));
    Duration::from_secs_f64(optimal_days * 24.0 * 60.0 * 60.0)
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
    let file = std::fs::File::open("/home/vboxuser/TestingCode/rustcompoundcalc/assets/alert.wav").unwrap(); // Make sure the path is correct
    let source = Decoder::new_wav(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end(); // Wait for the sound to finish
}

fn main() {
    let start_time = Instant::now();
    
    loop {
        clear_terminal();

        for (i, &balance) in BALANCES.iter().enumerate() {
            let time_to_compound = optimal_compounding_time(balance);
            let adjusted_elapsed = start_time.elapsed().as_secs() * SPEED;
            let remaining_time = time_to_compound.saturating_sub(Duration::from_secs(adjusted_elapsed));
            let percentage = 100.0 * (remaining_time.as_secs_f64() / time_to_compound.as_secs_f64());
            let progress_bar = print_progress_bar(percentage);

            println!(
                "Balance {}: ${:.2} | Time remaining: {} | {}",
                i + 1,
                balance,
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


use rand::seq::SliceRandom; // Import SliceRandom for shuffling
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use colored::*;
use std::process::Command;

fn main() -> io::Result<()> {
    // Parse the number of questions from the command line arguments
    let args: Vec<String> = env::args().collect();
    let num_questions: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10) // Default to 10 if the argument is not a valid number
    } else {
        10 // Default to 10 if no argument is provided
    };

    let file = File::open("/home/vboxuser/_coding/rustbucket/language_game/src/germandictionary.txt")?;
    let reader = BufReader::new(file);

    let mut questions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        if parts.len() == 6 {
            let question = format!("What is the German word for '{}'?", parts[0]);
            let correct_answer = parts[1].clone(); // Store the correct answer
            let mut options = vec![parts[1].clone(), parts[2].clone(), parts[3].clone(), parts[4].clone()];

            // Shuffle the options
            options.shuffle(&mut rand::thread_rng());

            // Find the new index of the correct answer after shuffling
            let new_correct_index = options.iter().position(|x| *x == correct_answer).expect("Correct answer not found");

            questions.push((question, options, new_correct_index));
        }
    }

    // Shuffle the questions
    questions.shuffle(&mut rand::thread_rng());

    let mut score = 0;
    let mut correct = 0;
    let mut incorrect = 0;

    // Limit the number of questions to the specified amount or the total number available
    let questions_to_ask = questions.iter().take(num_questions);

    for (i, (question, options, correct_index)) in questions_to_ask.enumerate() {
        println!("Question {}: {}", i + 1, question);

        loop {
            for (index, option) in options.iter().enumerate() {
                println!("{}. {}", index + 1, option);
            }

            print!("Please enter the number corresponding to your answer (1-4): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            match input.parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= 4 => {
                    if choice == *correct_index + 1 {
                        println!("{}", "Correct!\n".green());
                        score += 1;
                        correct += 1;
                    } else {
                        println!("{}\n", format!("Incorrect. The correct answer was {}", options[*correct_index]).red());
                        incorrect += 1;
                    }

                    // Construct the audio URL
                    let chosen_word = &options[choice - 1];
	        let audio_url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=de&client=gtx", urlencoding::encode(chosen_word));
		if let Err(e) = Command::new("ffplay")
   		 .arg("-nodisp")            // No display window
  		  .arg("-autoexit")          // Exit automatically after playing
  		  .arg(&audio_url)           // The audio URL
  		  .stdout(std::process::Stdio::null()) // Redirect stdout to /dev/null
   		 .stderr(std::process::Stdio::null())  // Redirect stderr to /dev/null
   		 .status() {
   		 eprintln!("Failed to play audio: {}", e);
		}
                    break; // Exit the loop after a valid input
                }
                _ => println!("Invalid input! Please enter a number between 1 and 4.\n"),
            }
        }

        println!("{}", format!("{}", score).bright_green().on_black());
    }

    println!("{}", format!("Your final score is: {} with {} incorrect answers", score, incorrect).bright_green().on_black());
    Ok(())
}


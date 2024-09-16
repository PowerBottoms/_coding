use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-download" {
        // Set the paths
        let download_list_path = "/home/vboxuser/_coding/rustbucket/language_game/src/GermanWordDownloadList";
        let download_folder_path = "/home/vboxuser/_coding/rustbucket/language_game/audios";

        // Read the GermanWordDownloadList file
        let download_list_file = File::open(download_list_path)?;
        let reader = BufReader::new(download_list_file);

        // Create a vector to store the words that need to be downloaded
        let mut words_to_download = Vec::new();

        // Get the list of already downloaded files (without extensions)
        let downloaded_files: Vec<String> = fs::read_dir(download_folder_path)?
            .filter_map(Result::ok)
            .filter_map(|entry| {
                if entry.path().is_file() {
                    entry.path().file_stem().map(|s| s.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect();

        // Compare the list of words to be downloaded with the already downloaded files
        for line in reader.lines() {
            let word = line?.trim().to_string();

            // Check if the word has already been downloaded
            if !downloaded_files.contains(&word) {
                words_to_download.push(word);
            }
        }

        // If no new words need to be downloaded, print a message and exit
        if words_to_download.is_empty() {
            println!("All words have been downloaded.");
            return Ok(());
        }

        // Download the missing words
        for word in words_to_download {
            let audio_url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=de&client=gtx", urlencoding::encode(&word));
            let output_file_path = format!("{}/{}.mp3", download_folder_path, word);

            // Use ffmpeg or another tool to download the audio
            let status = Command::new("ffmpeg")
                .arg("-i")
                .arg(&audio_url)
                .arg(&output_file_path)
                .status();

            match status {
                Ok(status) if status.success() => {
                    println!("Successfully downloaded '{}'", word);
                }
                Ok(_) | Err(_) => {
                    eprintln!("Failed to download '{}'", word);
                }
            }
        }

        Ok(())
    } else {
        println!("Usage: cargo run -- -download");
        Ok(())
    }
}


use std::process::Command;
use std::str;

pub fn get_nomic_balance() -> Result<f64, String> {
    // Run the `nomic balance` command
    let output = Command::new("nomic")
        .arg("balance")
        .arg("nomic1tvgzmmgy9lj3jvtqk2pagg0ng5rk8ajt5nc86u")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Check if the command was successful
    if output.status.success() {
        // Convert the command output to a string
        let output_str = str::from_utf8(&output.stdout).map_err(|e| format!("Failed to convert output to string: {}", e))?;

        // Example parsing logic (adjust based on actual output)
        if let Some(balance_line) = output_str.lines().find(|line| line.contains("NOM")) {
            // Assuming the balance number is the first part of the balance_line, split by whitespace
            if let Some(balance_str) = balance_line.split_whitespace().next() {
                // Parse the balance as a float, divide by 1,000,000 and return the result
                if let Ok(balance) = balance_str.parse::<f64>() {
                    let adjusted_balance = balance / 1_000_000.0;
                    return Ok(adjusted_balance);
                } else {
                    return Err("Failed to parse balance.".to_string());
                }
            }
        }

        Err("No balance information found.".to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
        Err(format!("Command failed with error: {}", stderr))
    }
}



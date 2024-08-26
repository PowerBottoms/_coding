use std::process::Command;
use crate::process;

pub fn display_help(){
	
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


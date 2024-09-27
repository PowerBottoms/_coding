use std::f64;

fn compound_strategy(initial_balance: f64, annual_rate: f64, compound_fee: f64, frequency: i32) -> (f64, f64, f64) {
    let periods_in_year = frequency as f64; // Frequency refers to number of compounds per year
    let period_rate = annual_rate / periods_in_year;
    let mut total_fee = 0.0;
    let mut claim_amount = 0.0;
    let mut balance = initial_balance;
    for _ in 0..frequency {
     claim_amount = balance * period_rate;
        balance += (balance * period_rate) - compound_fee;
        total_fee += compound_fee;
    }
    //println!(" Freq{} Balance {}", frequency ,balance);
    (balance, total_fee, claim_amount)
}

fn main() {
    let commission = 1.0 - 0.00;
    let initial_balance =  19782.0 * commission; // 11277.20   5716.73  19768
    let og_balance = initial_balance;
    let years = 1.0;
    let annual_rate = 0.26 * years; // 26% as a decimal
    let compound_fee = 0.01; // $0.01 fee per compound
    let mut first_claim = 0.0;
    let mut freqbuffer = 0;
    let mut bufftrigger: bool = false; // Buffer to print 10 extra calculations for comparison
    let mut previous_balance = initial_balance; // Start with initial balance

    // Start from frequency 1 up to every 30 minutes (i.e., frequency 17520 times/year)
    for frequency in 1..=17520 {
        let (final_balance, total_fee, claim_amount) = compound_strategy(initial_balance, annual_rate, compound_fee, frequency);
        let days_till_claim = (365.0 * years) / frequency as f64;
        
        println!("Frequency: {}, Final Balance: {:.4}, Fees: {:.2}, Claim amount {:.2}, Days till claim {:.2}", frequency, final_balance, total_fee, claim_amount, days_till_claim);
        
        if frequency == freqbuffer {break;} 
        
        // Check if the final balance has decreased from the previous frequency's final balance
        if frequency > 1 && final_balance < previous_balance && bufftrigger == false {
                    first_claim = (annual_rate / frequency as f64) * og_balance;
            println!("Stopped at frequency {} because the balance decreased from the previous frequency. First claim {:.2}.", frequency, first_claim);
            //break;
          
            freqbuffer = frequency + 10;
            bufftrigger = true;
        }
        previous_balance = final_balance; // Update previous balance for the next iteration
    }
    
}


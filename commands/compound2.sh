#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
RESET='\033[0m'
# Function to display the help section
display_help() {
    echo "Usage: $0 <principal> <fee> <freq> <interest> optional: <gains>"
    echo
    echo "This script calculates the future value of an investment over a specified frequency, accounting for interest and fees."
    echo
    echo "Arguments:"
    echo "  principal   The initial amount of money invested or loaned (e.g., 1000)."
    echo "  fee         The fee per period (e.g., 1.5)."
    echo "  freq        The number of periods to calculate the future value over (e.g., 365 for daily compounding over a year)."
    echo "  interest rate "
    echo
    echo "Options:"
    echo "  -h, --help  Display this help message and exit."
    echo
    echo "Example:"
    echo "  $0 1000 1.5 365"
    echo
    exit 0
}

# Check for help flag
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    display_help
fi

# Check if the correct number of arguments are provided
if [ "$#" -lt 4 ]; then
    echo -e "${RED}Usage: $0 <principal> <fee> <freq> <interest rate> optional:<gains>${RESET}"
    echo -e "${RED}Try '$0 --help' for more information.${RESET}"
    exit 1
fi

# Assign input arguments to variables
principal=$1
fee=$2
freq=$3
interest_rate=$4  # Example annual interest rate of 5% (0.05 as a decimal)
gains=$5
claimsaday=$(echo "scale=5; 365/$freq" | bc -l)
# Initialize future value with the initial principal
future_value_old=$principal
future_value_new=0
daily_percent=$(echo "scale=5; ($interest_rate/365)" |  bc -l)
percent_per_claim=$(echo "scale=5;  (1+ $daily_percent) / $freq" | bc -l )
# Loop to calculate future value over the specified frequency
for ((count=0; count<$freq; count++)); do
    future_value_new=$(echo "scale=5;  $future_value_old + (($future_value_old - $fee) * ( ( $interest_rate) /$freq) )" | bc -l)
    amountperclaim=$(echo "scale=5; $future_value_new - $future_value_old" | bc -l)
    claimnumber=$(echo "scale=5; $count + 1" | bc -l) 
    if [ "$gains" == "gains" ]; then
      echo -e "Claim: $claimnumber Yields: ${GREEN} $amountperclaim ${RESET} For a total of: ${GREEN} $future_value_new${RESET}"

    fi
   future_value_old=$future_value_new
done

gain=$(echo  "scale=5; $future_value_new - $principal" | bc -l)
claimamount=$(echo "scale=5; (0.26/($freq))*$principal  " | bc -l)
strattwo=$(echo "scale=5; (1.26 * $principal)" | bc -l)
strattwoclaim=$(echo "scale=5; ($interest_rate) * $principal  " | bc -l)
fees=$(echo "scale=5; $fee * $freq * (1+ $interest_rate)" | bc -l)
strat_diff=$(echo "scale=5; $future_value_old - $strattwo"| bc -l)
#  claims a day yields you claimamount
echo -e "Claiming every ${GREEN}$claimsaday${RESET} days. Yields you ${GREEN}$claimamount.${RESET} Losing ${RED}$fees${RESET} to fees. Which yielded you a net positive of ${BLUE}$strat_diff${RESET}"
# value after freq = future value old
echo -e "Final Future Value after ${GREEN}$freq${RESET} periods: ${GREEN}$future_value_old.${RESET}"
# gain from this strategy every claimsaday
echo -e "You gained ${BLUE}$principal${RESET} vs  ${BLUE}$future_value_new${RESET}."
#  strattwoclaim and strattwo total
echo -e "Compared to ${GREEN}1${RESET} claim of ${GREEN}$strattwoclaim${RESET} and a total of: ${GREEN}$strattwo.${RESET}"

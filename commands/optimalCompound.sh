#!/bin/bash

# Function to display the help section
display_help() {
    echo "Usage: $0 <principal> <apr> <fee>"
    echo
    echo "This script calculates the optimal number of compounding periods to maximize future value."
    echo
    echo "Arguments:"
    echo "  principal   The initial amount of money invested or loaned (e.g., 1000)."
    echo "  apr         The annual percentage rate (APR) as a decimal (e.g., 0.05 for 5%)."
    echo "  fee         The fee per compounding period (e.g., 1.5)."
    echo " count        The number of compounds you would like to calculate (e.g 365 for a year)"
    echo 
    echo "Options:"
    echo "  -h, --help  Display this help message and exit."
    echo
    echo "Example:"
    echo "  $0 1000 0.05 1.5 365"
    echo
    exit 0
}

# Check for help flag
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    display_help
fi

# Check if the correct number of arguments are provided
if [ "$#" -ne 4 ]; then
    echo "Usage: $0 <principal> <apr> <fee> <count>"
    echo "Try '$0 --help' for more information."
    exit 1
fi

# Assign input arguments to variables
principal=$1
apr=$2
fee=$3
max_n=$4  # Set the maximum number of compounding periods to test

# Initialize variables
max_fv=0
optimalfv=0
optimal_n=0

# Function to calculate future value
calculate_fv() {
    local n=$1
    local fv=$(echo "scale=10; $principal * (1 + $apr / $n)^$n - $n * $fee" | bc -l)
	echo $fv
}

# Loop through possible compounding periods
for ((n=1; n<=$max_n; n++)); do
    fv=$(calculate_fv $n)
    
    # Update the maximum FV and optimal n
    if (( $(echo "$fv > $max_fv" | bc -l) )); then
        max_fv=$fv
        optimal_n=$n
    fi
done
optimalfv=$(calculate_fv $max_n)

# Output the results
echo "Optimal Compounding Periods (n): $optimal_n"
echo "Optimal FV with n $optimalfv"
echo "Maximum Future Value (FV): $max_fv"


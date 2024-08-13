#!/bin/bash

# Run the nomic validators command and capture the output
output=$(nomic validators)

# Initialize total voting power
total_voting_power=0

# Process each line of the output
while IFS= read -r line; do
    # Extract the voting power value from each line
    # Assuming voting power is in a specific column or format; adjust as needed
    # Example: Extracting the last column which contains voting power
    voting_power=$(echo "$line" | awk '{print $NF}')
    
    # Add to total voting power
    if [[ $voting_power =~ ^[0-9]+$ ]]; then
        total_voting_power=$((total_voting_power + voting_power))
    fi
done <<< "$output"

# Calculate the total voting power divided by 1,000,000
result=$(echo "scale=6; $total_voting_power / 1000000" | bc)

# Print the result
echo "Total voting power divided by 1,000,000: $result"

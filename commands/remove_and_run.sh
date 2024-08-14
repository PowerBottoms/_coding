#!/bin/bash

# Path to the destination folder
DESTINATION="$HOME/Desktop/keysfolder"

# Initialize a counter
COUNTER=1

# Check if the destination folder exists, if not, create it
if [ ! -d "$DESTINATION" ]; then
  mkdir -p "$DESTINATION"
fi

# Find the highest existing folder number
if ls "$DESTINATION/keys"* 1> /dev/null 2>&1; then
  # Extract numbers from existing keys folders and find the maximum
  MAX=$(ls "$DESTINATION/keys"* | sed 's/[^0-9]*//g' | sort -n | tail -1)
  COUNTER=$((MAX + 1))
fi

while true; do
  # Create a new directory with the current counter value
  NEW_DIR="$DESTINATION/keys$COUNTER"
  mkdir -p "$NEW_DIR"

  # Move files to the new directory
  mv ~/nomic-stakenet-3/tendermint/config/priv_validator.json ~/nomic-stakenet-3/tendermint/config/node_key.json ~/nomic-stakenet-3/signer ~/.orga-wallet  "$NEW_DIR"

  # Increment the counter for the next run
  COUNTER=$((COUNTER + 1))
  #Run command
  nomic balance
  # Wait for half a second
  sleep 0.5
done





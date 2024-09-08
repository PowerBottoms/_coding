#!/bin/bash
# Usage: ./checkblocks.sh <number_of_blocks_to_check>
BLOCKS_TO_CHECK=$1

# Ensure BLOCKS_TO_CHECK is a valid number
if ! [[ "$BLOCKS_TO_CHECK" =~ ^[0-9]+$ ]]; then
    echo "Error: BLOCKS_TO_CHECK must be a positive integer."
    exit 1
fi

# Fetch the validator's address
VALIDATOR_ADDR=$(curl -s localhost:26657/status | jq -r .result.validator_info.address)
if [ -z "$VALIDATOR_ADDR" ]; then
    echo "Error: Could not fetch the validator address."
    exit 1
fi

# Fetch the last block height
LAST_BLOCK=$(curl -s localhost:26657/block | jq -r .result.block.header.height)
if ! [[ "$LAST_BLOCK" =~ ^[0-9]+$ ]]; then
    echo "Error: Could not fetch the last block height."
    exit 1
fi

# Calculate the start block
START_BLOCK=$(expr $LAST_BLOCK - $BLOCKS_TO_CHECK)
if ! [[ "$START_BLOCK" =~ ^[0-9]+$ ]]; then
    echo "Error: Start block calculation failed."
    exit 1
fi

# Debugging output
echo "Checking from block $START_BLOCK to $LAST_BLOCK"

# Loop through the blocks and check the signature
for BLOCK in $(seq "$START_BLOCK" "$LAST_BLOCK"); do
    RESULT=$(curl -s localhost:26657/block?height=$BLOCK | jq ".result.block.last_commit.signatures[] | select(.validator_address==\"$VALIDATOR_ADDR\").block_id_flag")
    
    if [ -z "$RESULT" ]; then
        echo "Block $BLOCK: no signature info"
    elif [ "$RESULT" == "2" ]; then
        echo "Block $BLOCK: signed"
    elif [ "$RESULT" == "1" ]; then
        echo "Block $BLOCK: not signed"
    else
        echo "Block $BLOCK: unknown signature flag: $RESULT"
    fi
done


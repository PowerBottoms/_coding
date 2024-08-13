#!/bin/bash

# File paths
file1="~/.nomic-*/config/priv_validator_key.json"
file2="~/.nomic-*/config/node_key.json"
file3="~/.nomic-*/signer"
file4="~/.orga-wallet"

# Remove files
if [ -e "$file1" ]; then
    rm "$file1"
    echo "Removed $file1"
else
    echo "$file1 does not exist"
fi

if [ -e "$file2" ]; then
    rm "$file2"
    echo "Removed $file2"
else
    echo "$file2 does not exist"
fi

if [ -e "$file3" ]; then
    rm "$file3"
    echo "Removed $file3"
else
    echo "$file3 does not exist"
fi

if [ -e "$file4" ]; then
    rm "$file4"
    echo "Removed $file4"
else
    echo "$file4 does not exist"
fi

#!/bin/bash

# Check if the -calc argument is supplied
if [[ " $@ " =~ " help " ]]; then

	echo "cmd.sh <calc>          Enables the rustcompoundcalc script "
        echo "cmd.sh <extract>       Extracts addresses and their balances of supplied address in the exporthandlerTest script"
        echo "cmd.sh <export>        Passes the command Nomic export to path ~/Desktop/nomicexport.txt supplied in cmd.sh"
fi

if [ "$1" == "calc" ]; then
    # Shift arguments to the left, effectively removing the first argument (calc)
    shift
    # Run the cargo command with the remaining arguments
    cargo run --manifest-path ~/TestingCode/rustcompoundcalc/Cargo.toml --bin rustcompoundcalc "$@"
fi
if [[ " $@ " =~ " extract " ]]; then
    cargo run --manifest-path ~/TestingCode/rustcompoundcalc/Cargo.toml --bin exporthandlerTest
    shift
fi
if [[ " $@ " =~ " timer " ]]; then
    cargo run --manifest-path ~/TestingCode/rustcompoundcalc/Cargo.toml --bin compoundtimer
    shift
fi
if [[ " $@ " =~ " export " ]]; then
    nomic export > ~/Desktop/Exports/nomicexport.txt
fi
if [[ " $@ " =~ " git it got it good " ]]; then
git status && git add . && git commit -m "Slim biggy" && git push && reset
fi

# Check for the "-pow" argument
if [[ " $@ " =~ " -pow " ]]; then
    # Calculate 36 to the power of 36 using bc
    result=$(echo "36^36" | bc)

    # Print the result
    echo "36 to the power of 36 is: $result"
fi




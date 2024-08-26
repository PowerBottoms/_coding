#!/bin/bash

# Function to display help
display_help() {
    echo "Usage: cmd.sh [options]"
    echo "cmd.sh <calc>          Enables the rustcompoundcalc script"
    echo "cmd.sh <extract>       Extracts addresses and their balances in the exporthandlerTest script"
    echo "cmd.sh <export>        Passes the command Nomic export to path ~/Desktop/nomicexport.txt"
    echo "cmd.sh <timer>         Passes the timer command which runs ~/_coding/rustcompoundcalc/Cargo.toml --bin compoundtimer"
    echo "cmd.sh <-pow>          The power of 36 to the 36"
}

# Check if no arguments are supplied
if [ $# -le 1 ]; then
    echo "Error: No arguments supplied."
    display_help
    exit 1
fi

# Handle different commands
for arg in "$@"; do
    case $arg in
        help)
            display_help
            exit 0
            ;;
        calc)
            shift
            cargo run --manifest-path ~/_coding/rustcompoundcalc/Cargo.toml --bin rustcompoundcalc "$@"
            exit 0
            ;;
        extract)
            cargo run --manifest-path ~/_coding/rustcompoundcalc/Cargo.toml --bin exporthandlerTest
            shift
            exit 0
            ;;
        timer)
            cargo run --manifest-path ~/_coding/rustcompoundcalc/Cargo.toml --bin compoundtimer
            shift
            exit 0
            ;;
        export)
            nomic export > ~/Desktop/Exports/nomicexport.txt
            exit 0
            ;;
        "-pow")
            result=$(echo "36^36" | bc)
            echo "36 to the power of 36 is: $result"
            exit 0
            ;;
        "git it got it good")
            git status && git add . && git commit -m "Slim biggy" && git push && reset
            exit 0
            ;;
        *)
            echo "Error: Unknown option '$arg'"
            display_help
            exit 1
            ;;
    esac
done


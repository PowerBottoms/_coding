#!/bin/bash

# Function to display help message
show_help() {
    echo "Usage: $0 <operation> [target_directory]"
    echo
    echo "Operations:"
    echo "  remove   Remove predefined files and directories from their locations"
    echo "  copy     Copy predefined files and directories to the target directory"
    echo "  move     Move predefined files and directories to the target directory"
    echo "  replace  Replace predefined files and directories from the target directory"
    echo
    echo "Arguments:"
    echo "  target_directory  Directory to which files and directories will be copied or moved, or from which they will be replaced"
    echo
    echo "Examples:"
    echo "  $0 remove"
    echo "  $0 copy /path/to/target_directory"
    echo "  $0 move /path/to/target_directory"
    echo "  $0 replace /path/to/target_directory"
}

# Check for -help argument
if [ "$#" -eq 1 ] && [ "$1" == "-help" ]; then
    show_help
    exit 0
fi

# Check if the correct number of arguments is provided
if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "Error: Incorrect number of arguments"
    show_help
    exit 1
fi

# Extract the operation and optional target directory from the command line arguments
operation=$1
target_directory=$2

# Predefined file and directory paths
file1="/home/vboxuser/.nomic-stakenet-3/tendermint/config/priv_validator_key.json"
file2="/home/vboxuser/.nomic-stakenet-3/tendermint/config/node_key.json"
dir1="/home/vboxuser/.nomic-stakenet-3/signer"
dir2="/home/vboxuser/.orga-wallet"

# Validate target directory for non-remove operations
if [ "$operation" != "remove" ]; then
    if [ -z "$target_directory" ]; then
        echo "Error: Target directory is required for $operation operation"
        show_help
        exit 1
    fi

    if [ ! -d "$target_directory" ]; then
        mkdir -p "$target_directory"
        echo "Created directory $target_directory"
    fi
fi

# Perform the specified operation
case "$operation" in
    remove)
        # Remove files
        for file in "$file1" "$file2"; do
            if [ -e "$file" ]; then
                rm "$file"
                echo "Removed $file"
            else
                echo "$file does not exist"
            fi
        done
        
        # Remove directories
        for dir in "$dir1" "$dir2"; do
            if [ -d "$dir" ]; then
                rm -r "$dir"
                echo "Removed directory $dir"
            else
                echo "$dir does not exist"
            fi
        done
        ;;
    copy)
        # Copy files
        for file in "$file1" "$file2"; do
            if [ -e "$file" ]; then
                cp "$file" "$target_directory"
                echo "Copied $file to $target_directory"
            else
                echo "$file does not exist"
            fi
        done
        
        # Copy directories
        for dir in "$dir1" "$dir2"; do
            if [ -d "$dir" ]; then
                cp -r "$dir" "$target_directory"
                echo "Copied directory $dir to $target_directory"
            else
                echo "$dir does not exist"
            fi
        done
        ;;
    move)
        # Move files
        for file in "$file1" "$file2"; do
            if [ -e "$file" ]; then
                mv "$file" "$target_directory"
                echo "Moved $file to $target_directory"
            else
                echo "$file does not exist"
            fi
        done
        
        # Move directories
        for dir in "$dir1" "$dir2"; do
            if [ -d "$dir" ]; then
                mv "$dir" "$target_directory"
                echo "Moved directory $dir to $target_directory"
            else
                echo "$dir does not exist"
            fi
        done
        ;;
    replace)
        # Replace files
        for file in "priv_validator_key.json" "node_key.json"; do
            target_file="$target_directory/$file"
            original_file_path=""
            
            case "$file" in
                "priv_validator_key.json")
                    original_file_path="$file1"
                    ;;
                "node_key.json")
                    original_file_path="$file2"
                    ;;
            esac

            if [ -e "$target_file" ]; then
                cp -r "$target_file" "$original_file_path"
                echo "Replaced $original_file_path with $target_file"
            else
                echo "$target_file does not exist"
            fi
        done
        
        # Replace directories
        for dir in "signer" ".orga-wallet"; do
            target_dir="$target_directory/$dir"
            original_dir_path=""
            
            case "$dir" in
                "signer")
                    original_dir_path="$dir1"
                    ;;
                ".orga-wallet")
                    original_dir_path="$dir2"
                    ;;
            esac

            if [ -d "$target_dir" ]; then
                cp -r "$target_dir" "$original_dir_path"
                echo "Replaced directory $original_dir_path with $target_dir"
            else
                echo "$target_dir does not exist"
            fi
        done
        ;;
    *)
        echo "Error: Unknown operation '$operation'"
        show_help
        exit 1
        ;;
esac

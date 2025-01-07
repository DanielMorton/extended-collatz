Cargo Run Loop Script

#!/bin/bash

# Check if correct number of arguments provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <starting_x> <number_of_iterations>"
    echo "Example: $0 0 10"
    exit 1
fi

# Get parameters from command line
x=$1
num_iterations=$2

# Validate inputs are numbers
if ! [[ "$x" =~ ^[0-9]+$ ]] || ! [[ "$num_iterations" =~ ^[0-9]+$ ]]; then
    echo "Error: Both arguments must be positive integers"
    exit 1
fi

echo "Starting loop with x=$x for $num_iterations iterations"

for ((i=0; i<num_iterations; i++)); do
    # Calculate end value (x + 4)
    end=$((x + 4))

    echo "Running iteration $i with start=$x, end=$end"

    # Run the cargo command
    cargo run --release -- -n 1048576 -s $x -e $end --write-cycle

    # Increment x by 6
    x=$((x + 6))

    # Optional: add a small delay between iterations
    sleep 1
done
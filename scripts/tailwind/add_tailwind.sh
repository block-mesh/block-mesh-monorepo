#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <name> <step_size>"
    exit 1
fi

# Assign arguments to variables
name=$1
step_size=$2

# Check if the step size is a positive integer
if ! [[ "$step_size" =~ ^[0-9]+$ ]] || [ "$step_size" -le 0 ]; then
    echo "Error: Step size must be a positive integer."
    exit 1
fi

# Loop from 0 to 100 with the specified step size
for ((i=0; i<=100; i+=step_size)); do
    echo "<div class='hidden'><div class='${name}-${i}'></div></div>" >> libs/block-mesh-manager/templates/tailwind-producer.html
done

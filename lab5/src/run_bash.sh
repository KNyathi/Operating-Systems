#!/bin/bash
# This script will run indefinitely, so we can track its memory map.

echo "Running Bash script with PID $$"
while true; do
  sleep 1  # Keeps the script running so we can track its memory usage.
done

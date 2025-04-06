import os
import re
import matplotlib.pyplot as plt
from datetime import datetime

def parse_memory_map(file_path):
    """
    Parse a memory map file and extract heap memory information.
    Returns the heap size in bytes.
    """
    with open(file_path, 'r') as f:
        lines = f.readlines()
        
    heap_size = 0
    heap_found = False
    
    for line in lines:
        # Look for a line containing the heap mapping
        if "heap" in line:
            heap_found = True
            # Extract size information (the third column is the size in bytes)
            size_match = re.search(r"([0-9a-f]+)-([0-9a-f]+)", line)
            if size_match:
                start_addr = int(size_match.group(1), 16)
                end_addr = int(size_match.group(2), 16)
                heap_size = end_addr - start_addr
                
    if not heap_found:
        print(f"No heap memory found in {file_path}")
        
    return heap_size

def extract_timestamp_from_filename(file_name):
    """
    Extract the timestamp from the filename using regex.
    Expected format: map_<pid>_<YYYY-MM-DD_HH:MM:SS>.txt
    """
    match = re.search(r"map_\d+_(\d{4}-\d{2}-\d{2}_\d{2}:\d{2}:\d{2})\.txt", file_name)
    if match:
        return match.group(1)
    return None

def plot_heap_sizes(map_directory):
    """
    Plot heap size changes over time based on the memory map files in a directory.
    """
    # Expand the user directory if ~ is present
    map_directory = os.path.expanduser(map_directory)
    
    heap_sizes = []
    timestamps = []
    
    # Get list of files in the directory
    for file_name in sorted(os.listdir(map_directory)):
        if file_name.endswith(".txt"):
            file_path = os.path.join(map_directory, file_name)
            
            # Extract timestamp using regex
            timestamp_str = extract_timestamp_from_filename(file_name)
            if not timestamp_str:
                print(f"Invalid timestamp format in file: {file_name}")
                continue  # Skip files with invalid format

            try:
                timestamp = datetime.strptime(timestamp_str, '%Y-%m-%d_%H:%M:%S')
            except ValueError:
                print(f"Failed to parse timestamp in file: {file_name}")
                continue

            # Parse the memory map file for heap size
            heap_size = parse_memory_map(file_path)
            
            # Store the data
            heap_sizes.append(heap_size)
            timestamps.append(timestamp)
    
    # Plot the graph
    if heap_sizes:
        plt.figure(figsize=(10, 6))
        plt.plot(timestamps, heap_sizes, marker='o', linestyle='-', color='b', label="Heap Size (bytes)")
        plt.xlabel("Time")
        plt.ylabel("Heap Size (bytes)")
        plt.title("Heap Size Changes Over Time")
        plt.xticks(rotation=45)
        plt.tight_layout()
        plt.grid(True)
        plt.legend()
        plt.show()
    else:
        print("No heap size data to plot.")

if __name__ == "__main__":
    # Set the directory containing the memory map files
    map_directory = input("Enter the directory containing memory map files: ")
    
    # Plot heap sizes
    plot_heap_sizes(map_directory)

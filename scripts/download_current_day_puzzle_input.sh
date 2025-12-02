#! /usr/bin/env bash

# Get current year and current day
year=$(date +%Y)
day=$(date +%d)

# Script can only be used during advent of code
if [[ ! $(date +%m) == 12 ]]; then
    echo "It's not December yet :("
    exit 2
fi

# Check if folder for today exists
if [[ ! -d "$year/day$day" ]]; then
    echo "Folder for $year/$day does not exist!"
    echo "Skipping download"
    exit 2
fi

# If so, download input there (if not exists, or is empty)
input_file="$year/day$day/input.txt"
if [[ ! -s $input_file ]]; then
    echo "Downloading $year/$day"
    aoc download --quiet --overwrite --year $year --day $day --input-only --input-file $input_file
fi

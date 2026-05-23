#!/usr/bin/env bash

year=$1
day=$2

if [[ -z "$year" || -z "$day" ]]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

target="day${day}_${year}"

# Compile target
cmake --build build --target "$target"

if [[ $? -ne 0 ]]; then
    echo "Build failed for $target"
    exit 1
fi

# Run target from it's directory
cd "$year/day$day" || exit
"../../build/$year/day$day/$target"
cd - > /dev/null || exit


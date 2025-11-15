#! /usr/bin/env nix-shell
#! nix-shell -i bash -p aoc-cli

for year_dir in */; do
    # Strip trailing slash
    year=${year_dir%/}

    # Skip directories not according to format
    [[ ! "$year" =~ ^[0-9]{4}$ ]] && continue

    for day_dir in $year/day*/; do
        # Get day number
        day=$(basename $day_dir)
        day=${day#day} # Remove 'day'

        # Test if input file exists
        input_file="$year/day$day/input.txt"
        if [[ ! -f $input_file ]]; then
            echo "Downloading input for $year/$day"
            # Missing input file, so run aoc cli to download it
            aoc download --quiet --year $year --day $day --input-only --input-file $input_file
        fi
    done
done

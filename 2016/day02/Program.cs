using System;
using System.Diagnostics;
using System.IO;
using System.Linq;

// Read input
Move[][] input = File.ReadAllLines("./input.txt")
    .Select(line =>
        line.Select(i => i switch {
            'U' => Move.Up,
            'D' => Move.Down,
            'L' => Move.Left,
            'R' => Move.Right,
            _ => throw new ArgumentException(),
        }).ToArray()
    )
    .ToArray();

// Part 1

var part1Code = "";
var currentNumber = 5;
foreach (Move[] nextNumberInstruction in input) {
    foreach (Move move in nextNumberInstruction) {
        switch (move) {
            case Move.Up:
                if (currentNumber > 3) {
                    currentNumber -= 3;
                }
                break;
            case Move.Down:
                if (currentNumber < 7) {
                    currentNumber += 3;
                }
                break;
            case Move.Left:
                if (currentNumber % 3 != 1) {
                    --currentNumber;
                }
                break;
            case Move.Right:
                if (currentNumber % 3 != 0) {
                    ++currentNumber;
                }
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    part1Code += currentNumber;
}

Console.WriteLine($"[Part 1] Bathroom code: {part1Code}");

// Part 2
currentNumber = 5;
var part2Code = "";
foreach (Move[] nextNumberInstruction in input) {
    foreach (Move move in nextNumberInstruction) {
        switch (move) {
            case Move.Up:
                if (currentNumber is not (5 or 2 or 1 or 4 or 9)) {
                    currentNumber -= currentNumber switch {
                        3 or 13 => 2,
                        _ => 4,
                    };
                }
                break;
            case Move.Down:
                if (currentNumber is not (5 or 10 or 13 or 12 or 9)) {
                    currentNumber += currentNumber switch {
                        1 or 11 => 2,
                        _ => 4,
                    };
                }
                break;
            case Move.Left:
                if (currentNumber is not (1 or 2 or 5 or 10 or 13)) {
                    --currentNumber;
                }
                break;
            case Move.Right:
                if (currentNumber is not (1 or 4 or 9 or 12 or 13)) {
                    ++currentNumber;
                }
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
        Debug.Assert(currentNumber is > 0 and <= 13);
    }

    if (currentNumber >= 10) {
        part2Code += (char) ((currentNumber - 10) + 'A');
    } else {
        part2Code += currentNumber;
    }
}
Console.WriteLine($"[Part 2] Bathroom code: {part2Code}");


// Classes

internal enum Move {
    Up, Down, Left, Right,
}

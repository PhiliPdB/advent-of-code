using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

// Read input
Instruction[] instructions = File.ReadAllText("./input.txt")
    .Split(", ")
    .Select(s => new Instruction {
        turn = s[0] == 'L' ? Turn.Left : Turn.Right,
        distance = int.Parse(s[1..])
    })
    .ToArray();

// Part 1
var currentDirection = Direction.North;
var currentPosition = (0, 0);

foreach (Instruction instruction in instructions) {
    currentDirection = instruction.turn switch {
        Turn.Left  => (Direction) (((int) currentDirection + 3) % 4),
        Turn.Right => (Direction) (((int) currentDirection + 1) % 4),
        _ => throw new ArgumentOutOfRangeException()
    };

    switch (currentDirection) {
        case Direction.North:
            currentPosition.Item2 -= instruction.distance;
            break;
        case Direction.East:
            currentPosition.Item1 += instruction.distance;
            break;
        case Direction.South:
            currentPosition.Item2 += instruction.distance;
            break;
        case Direction.West:
            currentPosition.Item1 -= instruction.distance;
            break;
        default:
            throw new ArgumentOutOfRangeException();
    }
}

Console.WriteLine($"[Part 1] Distance to HQ: {Math.Abs(currentPosition.Item1) + Math.Abs(currentPosition.Item2)}");


// Part 2
currentDirection = Direction.North;
currentPosition = (0, 0);

HashSet<(int, int)> visited = new() { currentPosition };

foreach (Instruction instruction in instructions) {
    currentDirection = instruction.turn switch {
        Turn.Left => (Direction)(((int)currentDirection + 3) % 4),
        Turn.Right => (Direction)(((int)currentDirection + 1) % 4),
        _ => throw new ArgumentOutOfRangeException()
    };

    for (var i = 0; i < instruction.distance; i++) {
        switch (currentDirection) {
            case Direction.North:
                currentPosition.Item2 -= 1;
                break;
            case Direction.East:
                currentPosition.Item1 += 1;
                break;
            case Direction.South:
                currentPosition.Item2 += 1;
                break;
            case Direction.West:
                currentPosition.Item1 -= 1;
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }

        if (!visited.Add(currentPosition)) {
            Console.WriteLine($"[Part 2] Distance to first visited twice: {Math.Abs(currentPosition.Item1) + Math.Abs(currentPosition.Item2)}");
            goto exit;
        }
    }
}

exit:
return 0;


internal enum Direction {
    North, East, South, West,
}

internal enum Turn {
    Left, Right,
}

internal struct Instruction {
    public Turn turn;
    public int distance;
}

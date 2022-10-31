
var moves = File.ReadAllText("./input.txt");

var part1Visited = new HashSet<(int, int)>();

var currentX = 0;
var currentY = 0;
foreach (char c in moves) {
    switch (c) {
        case '^':
            currentY--;
            break;
        case '>':
            currentX++;
            break;
        case 'v':
            currentY++;
            break;
        case '<':
            currentX--;
            break;
        default:
            throw new Exception("Invalid input");
    }

    part1Visited.Add((currentX, currentY));
}

Console.WriteLine($"[Part 1] Visited houses: {part1Visited.Count}");

var part2Visited = new HashSet<(int, int)>();

var santaX = 0;
var santaY = 0;
var roboSantaX = 0;
var roboSantaY = 0;

for (var i = 0; i < moves.Length; i += 2) {
    switch (moves[i]) {
        case '^':
            santaY--;
            break;
        case '>':
            santaX++;
            break;
        case 'v':
            santaY++;
            break;
        case '<':
            santaX--;
            break;
        default:
            throw new Exception("Invalid input");
    }
    switch (moves[i + 1]) {
        case '^':
            roboSantaY--;
            break;
        case '>':
            roboSantaX++;
            break;
        case 'v':
            roboSantaY++;
            break;
        case '<':
            roboSantaX--;
            break;
        default:
            throw new Exception("Invalid input");
    }

    part2Visited.Add((santaX, santaY));
    part2Visited.Add((roboSantaX, roboSantaY));
}

Console.WriteLine($"[Part 2] Visited houses: {part2Visited.Count}");

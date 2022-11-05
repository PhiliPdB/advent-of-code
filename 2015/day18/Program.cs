
var onLights = new HashSet<(int x, int y)>();

var lines = File.ReadAllLines("./input.txt");
void ResetOnLights() {
    onLights.Clear();
    for (var y = 0; y < lines.Length; y++) {
        for (var x = 0; x < lines[y].Length; x++) {
            if (lines[y][x] == '#') {
                onLights.Add((x, y));
            }
        }
    }
}

HashSet<(int x, int y)> NextStep(bool stuckCornerLights = false) {
    var newOnLights = new HashSet<(int x, int y)>(onLights.Count);
    if (stuckCornerLights) {
        newOnLights.Add((0, 0));
        newOnLights.Add((0, lines.Length - 1));
        newOnLights.Add((lines[0].Length - 1, 0));
        newOnLights.Add((lines[0].Length - 1, lines.Length - 1));
    }

    foreach ((int x, int y) in onLights) {
        // Check neighbours to see if this light stays on
        int onNeighbours = Neighbours(x, y).Count(n => onLights.Contains(n));
        if (onNeighbours is 2 or 3) {
            newOnLights.Add((x, y));
        }

        // Check neighbours that are off te see if they should turn on
        foreach ((int nx, int ny) in Neighbours(x, y)) {
            if (nx < 0 || nx >= lines[0].Length || ny < 0 || ny >= lines.Length) {
                // Neighbour outside of the grid
                continue;
            }
            if (onLights.Contains((nx, ny))) {
                // Neighbour should be off
                continue;
            }

            onNeighbours = Neighbours(nx, ny).Count(n => onLights.Contains(n));
            if (onNeighbours == 3) {
                newOnLights.Add((nx, ny));
            }
        }
    }

    return newOnLights;
}

static (int x, int y)[] Neighbours(int x, int y) {
    return new[] {
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    };
}

// Part 1
ResetOnLights();
for (var step = 0; step < 100; step++) {
    onLights = NextStep();
}
Console.WriteLine($"[Part 1] Total lights on: {onLights.Count}");

// Part 2
ResetOnLights();
onLights.Add((0, 0));
onLights.Add((0, lines.Length - 1));
onLights.Add((lines[0].Length - 1, 0));
onLights.Add((lines[0].Length - 1, lines.Length - 1));

for (var step = 0; step < 100; step++) {
    onLights = NextStep(true);
}
Console.WriteLine($"[Part 2] Total lights on: {onLights.Count}");

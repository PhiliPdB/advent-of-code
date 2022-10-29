using System.Text.RegularExpressions;
using day22;

const string pattern = @"^\/dev\/grid\/node-x(\d+)-y(\d+)\s*(\d+)T\s*(\d+)T\s*(\d+)T\s*(\d+)\%$";
Node[] nodes = File.ReadAllLines("./input.txt")
    .Skip(2)
    .Select(line => {
        GroupCollection groups = Regex.Matches(line, pattern)[0].Groups;
        return new Node(
            int.Parse(groups[1].Value), 
            int.Parse(groups[2].Value),
            int.Parse(groups[3].Value),
            int.Parse(groups[4].Value)
        );
    })
    .ToArray();

// Part 1
var numberOfViablePairs = 0;
for (var i = 0; i < nodes.Length; i++) {
    if (nodes[i].Usage == 0) {
        continue;
    }

    for (var j = 0; j < nodes.Length; j++) {
        if (i == j) {
            continue;
        }

        if (nodes[j].Usage + nodes[i].Usage <= nodes[j].Capacity) {
            numberOfViablePairs++;
        }
    }
}
Console.WriteLine($"[Part 1] Number of viable pairs: {numberOfViablePairs}");

// Part 2
int maxX = nodes.Max(n => n.X);
int maxY = nodes.Max(n => n.Y);

// The initial empty spot
var emptySpot = (0, 0);
// Locations that contain so much data, they cannot be moved around.
var blockedLocations = new HashSet<(int, int)>();
foreach (Node n in nodes) {
    if (n.Usage == 0) {
        emptySpot = (n.X, n.Y);
    }

    if (n.Usage > 100) {
        blockedLocations.Add((n.X, n.Y));
    }
}


var queue = new Queue<((int x, int y) dataLocation, (int x, int y) emptySpot, int steps)>();
queue.Enqueue(((maxX, 0), emptySpot, 0));
var visited = new HashSet<((int, int) data, (int, int) emptySpot)>();

// BFS
while (queue.TryDequeue(out var res)) {
    // Check end if we are at the end location
    if (res.dataLocation == (0, 0)) {
        Console.WriteLine($"[Part 2] Fewest steps: {res.steps}");
        break;
    }

    if (!visited.Add((res.dataLocation, res.emptySpot))) {
        continue;
    }

    // Generate new moves

    if (res.emptySpot.x > 0) {
        var newEmptySpot = (res.emptySpot.x - 1, res.emptySpot.y);
        if (!blockedLocations.Contains(newEmptySpot)) {
            var dataLocation = res.dataLocation == newEmptySpot ? res.emptySpot : res.dataLocation;
            queue.Enqueue((dataLocation, newEmptySpot, res.steps + 1));
        }
    }
    if (res.emptySpot.x < maxX) {
        var newEmptySpot = (res.emptySpot.x + 1, res.emptySpot.y);
        if (!blockedLocations.Contains(newEmptySpot)) {
            var dataLocation = res.dataLocation == newEmptySpot ? res.emptySpot : res.dataLocation;
            queue.Enqueue((dataLocation, newEmptySpot, res.steps + 1));
        }
    }

    if (res.emptySpot.y > 0) {
        var newEmptySpot = (res.emptySpot.x, res.emptySpot.y - 1);
        if (!blockedLocations.Contains(newEmptySpot)) {
            var dataLocation = res.dataLocation == newEmptySpot ? res.emptySpot : res.dataLocation;
            queue.Enqueue((dataLocation, newEmptySpot, res.steps + 1));
        }
    }

    if (res.emptySpot.y < maxY) {
        var newEmptySpot = (res.emptySpot.x, res.emptySpot.y + 1);
        if (!blockedLocations.Contains(newEmptySpot)) {
            var dataLocation = res.dataLocation == newEmptySpot ? res.emptySpot : res.dataLocation;
            queue.Enqueue((dataLocation, newEmptySpot, res.steps + 1));
        }
    }
}

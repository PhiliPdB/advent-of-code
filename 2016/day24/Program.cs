
var pointsOfInterest = new (int x, int y)[8];
var grid = File.ReadAllLines("./input.txt")
    .Select((line, y) => {
        var row = new List<bool>();
        for (var x = 0; x < line.Length; x++) {
            if (line[x] == '#') {
                row.Add(false);
            } else if (line[x] == '.') {
                row.Add(true);
            } else {
                row.Add(true);
                pointsOfInterest[int.Parse(line[x].ToString())] = (x, y);
            }
        }
        return row.ToArray();
    })
    .ToArray();

var poiLookup = new Dictionary<(int, int), int>();
for (var i = 0; i < pointsOfInterest.Length; i++) {
    poiLookup[pointsOfInterest[i]] = i;
}

var queue = new Queue<((int x, int y) position, int steps, int visitedPOIs)>();
queue.Enqueue((pointsOfInterest[0], 0, 1));
var visited = new HashSet<((int, int) position, int visitedPOIs)>();

int? part1Steps = null;
int? part2Steps = null;
while (queue.TryDequeue(out var res)) {
    if (res.visitedPOIs == 0b11111111) {
        part1Steps ??= res.steps;
        if (res.position == pointsOfInterest[0]) {
            part2Steps = res.steps;
            break;
        }
    }

    if (!visited.Add((res.position, res.visitedPOIs))) {
        continue;
    }

    if (grid[res.position.y - 1][res.position.x]) {
        var newPosition = (res.position.x, res.position.y - 1);
        var newVisitedPOIs = res.visitedPOIs;
        if (poiLookup.TryGetValue(newPosition, out int index)) {
            newVisitedPOIs |= (1 << index);
        }
        queue.Enqueue((newPosition, res.steps + 1, newVisitedPOIs));
    }
    if (grid[res.position.y + 1][res.position.x]) {
        var newPosition = (res.position.x, res.position.y + 1);
        var newVisitedPOIs = res.visitedPOIs;
        if (poiLookup.TryGetValue(newPosition, out int index)) {
            newVisitedPOIs |= (1 << index);
        }
        queue.Enqueue((newPosition, res.steps + 1, newVisitedPOIs));
    }

    if (grid[res.position.y][res.position.x - 1]) {
        var newPosition = (res.position.x - 1, res.position.y);
        var newVisitedPOIs = res.visitedPOIs;
        if (poiLookup.TryGetValue(newPosition, out int index)) {
            newVisitedPOIs |= (1 << index);
        }
        queue.Enqueue((newPosition, res.steps + 1, newVisitedPOIs));
    }
    if (grid[res.position.y][res.position.x + 1]) {
        var newPosition = (res.position.x + 1, res.position.y);
        var newVisitedPOIs = res.visitedPOIs;
        if (poiLookup.TryGetValue(newPosition, out int index)) {
            newVisitedPOIs |= (1 << index);
        }
        queue.Enqueue((newPosition, res.steps + 1, newVisitedPOIs));
    }
}

Console.WriteLine($"[Part 1] Fewest steps: {part1Steps}");
Console.WriteLine($"[Part 2] Fewest steps: {part2Steps}");


var currentIndex = 0;
var locationLookup = new Dictionary<string, int>();
var adjacencyMatrix = new List<List<(int, int)>>();

foreach (var line in File.ReadAllLines("./input.txt")) {
    var costSplit = line.Split(" = ");
    var locationSplit = costSplit[0].Split(" to ");

    if (!locationLookup.ContainsKey(locationSplit[0])) {
        locationLookup[locationSplit[0]] = currentIndex;
        adjacencyMatrix.Add(new List<(int, int)>());
        currentIndex++;
    }
    if (!locationLookup.ContainsKey(locationSplit[1])) {
        locationLookup[locationSplit[1]] = currentIndex;
        adjacencyMatrix.Add(new List<(int, int)>());
        currentIndex++;
    }

    adjacencyMatrix[locationLookup[locationSplit[0]]].Add((locationLookup[locationSplit[1]], int.Parse(costSplit[1])));
    adjacencyMatrix[locationLookup[locationSplit[1]]].Add((locationLookup[locationSplit[0]], int.Parse(costSplit[1])));
}


// Part 1

int ShortestPath(int startNode) {
    var queue = new PriorityQueue<(int node, int visitedNodes), int>();
    var visited = new HashSet<(int, int)>();

    queue.Enqueue((startNode, 1 << startNode), 0);
    while (queue.TryDequeue(out var current, out int length)) {
        if (current.visitedNodes == 0b11111111) {
            return length;
        }

        if (!visited.Add(current)) {
            continue;
        }

        foreach ((int nextNode, int l) in adjacencyMatrix[current.node]) {
            if (((current.visitedNodes >> nextNode) & 1) == 0) {
                queue.Enqueue((nextNode, current.visitedNodes | (1 << nextNode)), length + l);
            }
        }
    }

    throw new Exception("Unreachable");
}

int shortestDistance = Enumerable.Range(0, currentIndex)
    .Min(ShortestPath);
Console.WriteLine($"[Part 1] Shortest path: {shortestDistance}");

// Part 2

int LongestPath(int startNode) {
    var queue = new PriorityQueue<(int node, int visitedNodes), int>();
    var visited = new HashSet<(int, int)>();

    queue.Enqueue((startNode, 1 << startNode), 0);
    while (queue.TryDequeue(out var current, out int length)) {
        if (current.visitedNodes == 0b11111111) {
            return -length;
        }

        if (!visited.Add(current)) {
            continue;
        }

        foreach ((int nextNode, int l) in adjacencyMatrix[current.node]) {
            if (((current.visitedNodes >> nextNode) & 1) == 0) {
                queue.Enqueue((nextNode, current.visitedNodes | (1 << nextNode)), length - l);
            }
        }
    }

    throw new Exception("Unreachable");
}

int longestDistance = Enumerable.Range(0, currentIndex)
    .Max(LongestPath);
Console.WriteLine($"[Part 2] Longest path: {longestDistance}");


const int favoriteNumber = 1364;

bool IsWall(int x, int y) {
    int sum = x*x + 3*x + 2*x*y + y + y*y + favoriteNumber;

    var count = 0;
    while (sum != 0) {
        count++;
        sum &= sum - 1;
    }
    return count % 2 == 1;
}

(int steps, int locationsWithin50Steps) ReachLocation(Coordinate endLocation) {
    var locationsWithin50Steps = 0;

    var queue = new Queue<(Coordinate coord, int steps)>();
    queue.Enqueue((new Coordinate(0, 0), 0));

    var visited = new HashSet<Coordinate>();

    while (queue.TryDequeue(out (Coordinate coord, int steps) res)) {
        if (res.coord == endLocation) {
            return (res.steps, locationsWithin50Steps);
        }

        if (!visited.Add(res.coord)) {
            continue;
        }

        if (res.steps < 50) {
            locationsWithin50Steps++;
        }

        if (res.coord.X > 0 && !IsWall(res.coord.X - 1, res.coord.Y)) {
            // Move west
            queue.Enqueue((res.coord with { X = res.coord.X - 1 }, res.steps + 1));
        }

        if (res.coord.Y > 0 && !IsWall(res.coord.X, res.coord.Y - 1)) {
            // Move north
            queue.Enqueue((res.coord with { Y = res.coord.Y - 1 }, res.steps + 1));
        }

        if (!IsWall(res.coord.X + 1, res.coord.Y)) {
            // Move east
            queue.Enqueue((res.coord with { X = res.coord.X + 1 }, res.steps + 1));
        }

        if (!IsWall(res.coord.X, res.coord.Y + 1)) {
            // Move south
            queue.Enqueue((res.coord with { Y = res.coord.Y + 1 }, res.steps + 1));
        }
    }

    throw new Exception("Unreachable");
}


(int steps, int locationsWithin50Steps) = ReachLocation(new Coordinate(31, 39));
// Part 1
Console.WriteLine($"[Part 1] Reached destination in {steps} steps.");
// Part 2
Console.WriteLine($"[Part 2] Locations within 50 steps: {locationsWithin50Steps}");


public readonly record struct Coordinate(int X, int Y);

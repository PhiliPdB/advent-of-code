using System.Security.Cryptography;
using System.Text;

using var md5Hash = MD5.Create();

const string passCode = "rrrbmfta";
string DoorStates(string path) {
    byte[] hashBytes = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(passCode + path));
    return Convert.ToHexString(hashBytes)[..4].ToLower();
}

(string shortestPath, int longestPathLength) PathsToVault() {
    var queue = new Queue<(Coordinate coord, int steps, string path)>();
    queue.Enqueue((new Coordinate(0, 0), 0, ""));

    string? shortestPathString = null;
    var longestPathLength = 0;

    while (queue.TryDequeue(out (Coordinate coord, int steps, string path) res)) {
        if (res.coord == new Coordinate(3, 3)) {
            shortestPathString ??= res.path;

            if (res.steps > longestPathLength) {
                longestPathLength = res.steps;
            }
            continue;
        }

        // Generate new moves
        string doorStates = DoorStates(res.path);
        if (res.coord.X > 0 && doorStates[2] > 'a') {
            queue.Enqueue((res.coord with { X = res.coord.X - 1 }, res.steps + 1, res.path + 'L'));
        }
        if (res.coord.X < 3 && doorStates[3] > 'a') {
            queue.Enqueue((res.coord with { X = res.coord.X + 1 }, res.steps + 1, res.path + 'R'));
        }

        if (res.coord.Y > 0 && doorStates[0] > 'a') {
            queue.Enqueue((res.coord with { Y = res.coord.Y - 1 }, res.steps + 1, res.path + 'U'));
        }
        if (res.coord.Y < 3 && doorStates[1] > 'a') {
            queue.Enqueue((res.coord with { Y = res.coord.Y + 1 }, res.steps + 1, res.path + 'D'));
        }
    }

    return (shortestPathString!, longestPathLength);
}


// Part 1
(string shortestPath, int longestPathLength) = PathsToVault();
Console.WriteLine($"[Part 1] Path: {shortestPath}");
Console.WriteLine($"[Part 2] Longest path length: {longestPathLength}");


public readonly record struct Coordinate(int X, int Y);


// Read input
int[][] input = File.ReadAllLines("./input.txt")
    .Select(line => line.Split()
        .Where(l => l != "")
        .Select(int.Parse)
        .ToArray()
    )
    .ToArray();

// Part 1
int part1ValidTriangles = input
    .Count(t => t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0]);
Console.WriteLine($"[Part 1] Valid triangles: {part1ValidTriangles}");

// Part 2
var part2ValidTriangles = 0;
for (var i = 0; i < input.Length; i += 3) {
    for (var j = 0; j < 3; ++j) {
        if (input[i][j] + input[i + 1][j] > input[i + 2][j]
            && input[i][j] + input[i + 2][j] > input[i + 1][j]
            && input[i + 1][j] + input[i + 2][j] > input[i][j]
           ) {
            ++part2ValidTriangles;
        }
    }
}
Console.WriteLine($"[Part 2] Valid triangles: {part2ValidTriangles}");


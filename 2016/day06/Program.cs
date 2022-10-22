
string[] input = File.ReadAllLines("./input.txt");

List<char[]> columns = new();
for (var i = 0; i < input[0].Length; i++) {
    columns.Add(input.Select(l => l[i]).ToArray());
}


// Part 1
string part1ErrorCorrectedMessage = string.Concat(
    columns.Select(
        column => column.GroupBy(c => c)
            .OrderByDescending(g => g.Count())
            .First().Key
    )
);
Console.WriteLine($"[Part 1] Message: {part1ErrorCorrectedMessage}");

// Part 2
string part2ErrorCorrectedMessage = string.Concat(
    columns.Select(
        column => column.GroupBy(c => c)
            .OrderBy(g => g.Count())
            .First().Key
    )
);
Console.WriteLine($"[Part 2] Message: {part2ErrorCorrectedMessage}");

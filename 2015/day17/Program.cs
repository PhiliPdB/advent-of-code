// NOTE: Coin Change problem
// Except we can use each coin only once

var capacities = File.ReadAllLines("./input.txt").Select(int.Parse).ToArray();


// S(n, m) = 0 if n < 0 or (n > 0 and no containers left)
// S(0, m) = 1
// S(n, m) = S(n, m + 1) + S(n - c[m], m + 1)
(int ways, int minContainers) Combinations(int N, int i, int containers, int maxContainers = int.MaxValue) {
    if (N < 0 || (N > 0 && i >= capacities.Length) || containers > maxContainers) {
        return (0, int.MaxValue);
    }
    if (N == 0) {
        return (1, containers);
    }

    var withoutI = Combinations(N, i + 1, containers, maxContainers);
    var withI = Combinations(N - capacities[i], i + 1, containers + 1, maxContainers);

    return (withoutI.ways + withI.ways, Math.Min(withoutI.minContainers, withI.minContainers));
}

var part1Result = Combinations(150, 0, 0);
Console.WriteLine($"[Part 1] Total combinations: {part1Result.ways}");

var part2Result = Combinations(150, 0, 0, part1Result.minContainers);
Console.WriteLine($"[Part 2] Total combinations: {part2Result.ways}");

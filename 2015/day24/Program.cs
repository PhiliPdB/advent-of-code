
var weights = File.ReadAllLines("./input.txt")
    .Select(int.Parse)
    .OrderDescending()
    .ToArray();
int totalWeight = weights.Sum();

// Calculate subsets

var subsets = new List<List<int>>();
void Subsets(int from, List<int> items, int sum, int targetSum, int maxItems = 6) {
    if (sum == targetSum) {
        subsets.Add(items);
        return;
    }

    if (from >= weights.Length || sum > targetSum || items.Count > maxItems) {
        return;
    }

    var newItems = new List<int>(items) { weights[from] };
    Subsets(from + 1, newItems, sum + weights[from], targetSum, maxItems);
    Subsets(from + 1, items, sum, targetSum, maxItems);
}

// Part 1

var size = 1;
var part1Target = totalWeight / 3;
while (subsets.Count == 0) {
    Subsets(0, new List<int>(), 0, part1Target, size);
    size++;
}

var part1Result = subsets.Select(ss => ss.Aggregate(1L, (current, i) => current * i)).Order().First();
Console.WriteLine($"[Part 1] Least QE: {part1Result}");


// Part 2

subsets.Clear();
size = 1;
var part2Target = totalWeight / 4;
while (subsets.Count == 0) {
    Subsets(0, new List<int>(), 0, part2Target, size);
    size++;
}

var part2Result = subsets.Select(ss => ss.Aggregate(1L, (current, i) => current * i)).Order().First();
Console.WriteLine($"[Part 2] Least QE: {part2Result}");

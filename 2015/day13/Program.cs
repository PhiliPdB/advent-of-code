
var lines = File.ReadAllLines("./input.txt");

var peopleSet = new HashSet<string>();
var preferences = new Dictionary<(string from, string to), int>();
foreach (string line in lines) {
    var split = line[..^1].Split(' ');

    peopleSet.Add(split[0]);
    peopleSet.Add(split[^1]);

    int happiness = int.Parse(split[3]);
    if (split[2] == "lose") {
        happiness *= -1;
    }

    if (!preferences.ContainsKey((split[0], split[^1]))) {
        preferences[(split[0], split[^1])] = 0;
    }
    if (!preferences.ContainsKey((split[^1], split[0]))) {
        preferences[(split[^1], split[0])] = 0;
    }

    preferences[(split[0], split[^1])] += happiness;
    preferences[(split[^1], split[0])] += happiness;
}
List<string> people = peopleSet.ToList();


int OrderScore(int[] order) {
    var score = 0;
    for (var i = 0; i < order.Length - 1; i++) {
        score += preferences[(people[order[i]], people[order[i + 1]])];
    }
    score += preferences[(people[order[^1]], people[order[0]])];
    return score;
}

var part1Orderings = Permute(Enumerable.Range(0, people.Count).ToArray(), 1, people.Count, new List<int[]>());
Console.WriteLine($"[Part 1] Found a happiness of: {part1Orderings.Max(OrderScore)}");


var part2Orderings = Permute(Enumerable.Range(0, people.Count + 1).ToArray(), 1, people.Count + 1, new List<int[]>());
foreach (string p in people) {
    preferences[(p, "me")] = 0;
    preferences[("me", p)] = 0;
}
people.Add("me");
Console.WriteLine($"[Part 2] Found a happiness of: {part2Orderings.Max(OrderScore)}");



static List<int[]> Permute(int[] names, int start, int end, List<int[]> partialResults) {
    if (start == end) {
        partialResults.Add(names.ToArray());
    } else {
        for (int i = start; i < end; i++) {
            (names[start], names[i]) = (names[i], names[start]);
            Permute(names, start + 1, end, partialResults);
            (names[start], names[i]) = (names[i], names[start]);
        }
    }

    return partialResults;
};

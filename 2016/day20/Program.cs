
List<(uint lower, uint upper)> ips = File.ReadAllLines("./input.txt")
    .Select(line => {
        string[] split = line.Split('-');
        return (uint.Parse(split[0]), uint.Parse(split[1]));
    })
    .ToList();
ips.Sort((a, b) => a.lower.CompareTo(b.lower));

// Part 1
uint lastPotentialValid = 0;
uint? firstValid = null;
uint totalValid = 0;

for (var i = 0; i < ips.Count; i++) {
    if (lastPotentialValid < ips[i].lower) {
        totalValid += ips[i].lower - lastPotentialValid;

        firstValid ??= lastPotentialValid;
    }

    if (ips[i].upper == uint.MaxValue) {
        break;
    }
    lastPotentialValid = Math.Max(lastPotentialValid, ips[i].upper + 1);
}

Console.WriteLine($"[Part 1] First accepted ip: {firstValid}");
Console.WriteLine($"[Part 2] Total accepted ips: {totalValid}");

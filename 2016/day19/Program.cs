
const int numberOfElves = 3_012_210;

// Part 1
var start = 1;
var stepSize = 1;
int currentNumberOfElves = numberOfElves;
while (currentNumberOfElves != 1) {
    stepSize *= 2;

    if (currentNumberOfElves % 2 != 0) {
        start += stepSize;
    }

    currentNumberOfElves /= 2;
}

Console.WriteLine($"[Part 1] Elve receiving everything: {start}");

// Part 2
List<int> elves = Enumerable.Range(1, numberOfElves)
    // If number of elves is divisible by three, every third elve is kept.
    .Where(i => i % 9 == 0)
    .ToList();

while (elves.Count > 1) {
    var index = 0;
    while (index < elves.Count) {
        int removeAt = (index + elves.Count / 2) % elves.Count;
        elves.RemoveAt(removeAt);

        if (removeAt > index) {
            index++;
        }
    }
}
Console.WriteLine($"[Part 2] Elve receiving everything: {elves[0]}");

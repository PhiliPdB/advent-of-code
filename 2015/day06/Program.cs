
var instructions = File.ReadAllLines("./input.txt");

// Part 1
var lights = new bool[1000, 1000];
foreach (string instruction in instructions) {
    string[] splitted = instruction.Split(' ');
    string[] start = splitted[^3].Split(',');
    string[] end = splitted[^1].Split(',');

    int startX = int.Parse(start[0]);
    int startY = int.Parse(start[1]);
    int endX = int.Parse(end[0]);
    int endY = int.Parse(end[1]);

    if (instruction.StartsWith("turn on")) {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                lights[x, y] = true;
            }
        }
    } else if (instruction.StartsWith("turn off")) {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                lights[x, y] = false;
            }
        }
    } else {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                lights[x, y] = !lights[x, y];
            }
        }
    }
}

var lightsOn = 0;
for (var x = 0; x < 1000; x++) {
    for (var y = 0; y < 1000; y++) {
        if (lights[x, y]) {
            lightsOn++;
        }
    }
}
Console.WriteLine($"[Part 1] Lights on: {lightsOn}");

// Part 2
var lightBrightness = new int[1000, 1000];
foreach (string instruction in instructions) {
    string[] splitted = instruction.Split(' ');
    string[] start = splitted[^3].Split(',');
    string[] end = splitted[^1].Split(',');

    int startX = int.Parse(start[0]);
    int startY = int.Parse(start[1]);
    int endX = int.Parse(end[0]);
    int endY = int.Parse(end[1]);

    if (instruction.StartsWith("turn on")) {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                lightBrightness[x, y]++;
            }
        }
    } else if (instruction.StartsWith("turn off")) {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                if (lightBrightness[x, y] > 0) {
                    lightBrightness[x, y]--;
                }
            }
        }
    } else {
        for (int x = startX; x <= endX; x++) {
            for (int y = startY; y <= endY; y++) {
                lightBrightness[x, y] += 2;
            }
        }
    }
}

var totalBrightness = 0;
for (var x = 0; x < 1000; x++) {
    for (var y = 0; y < 1000; y++) {
        totalBrightness += lightBrightness[x, y];
    }
}
Console.WriteLine($"[Part 2] Total brightness: {totalBrightness}");

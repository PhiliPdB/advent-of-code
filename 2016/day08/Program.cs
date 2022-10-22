
const int width = 50;
const int height = 6;

var screen = new bool[height, width];

string[] instructions = File.ReadAllLines("./input.txt");
foreach (string instruction in instructions) {
    string[] parts = instruction.Split(' ');
    if (parts[0] == "rect") {
        int maxX = int.Parse(parts[1].Split('x')[0]);
        int maxY = int.Parse(parts[1].Split('x')[1]);
        for (var y = 0; y < maxY; y++) {
            for (var x = 0; x < maxX; x++) {
                screen[y, x] = true;
            }
        }
    } else if (parts[0] == "rotate" && parts[1] == "row") {
        int column = int.Parse(parts[2][2..]);
        int by = int.Parse(parts[4]);

        List<bool> right = new();
        for (var i = 0; i < by; i++) {
            right.Add(screen[column, width - by + i]);
        }

        for (int i = width - 1; i >= by; i--) {
            screen[column, i] = screen[column, i - by];
        }
        for (var i = 0; i < by; i++) {
            screen[column, i] = right[i];
        }
    } else if (parts[0] == "rotate" && parts[1] == "column") {
        int row = int.Parse(parts[2][2..]);
        int by = int.Parse(parts[4]);

        List<bool> bottom = new();
        for (var i = 0; i < by; i++) {
            bottom.Add(screen[height - by + i, row]);
        }

        for (int i = height - 1; i >= by; i--) {
            screen[i, row] = screen[i - by, row];
        }
        for (var i = 0; i < by; i++) {
            screen[i, row] = bottom[i];
        }
    }
}

var onPixels = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (screen[y, x]) {
            onPixels++;
        }
    }
}
Console.WriteLine($"[Part 1] Lit pixels: {onPixels}");


Console.WriteLine();
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        Console.Write(screen[y, x] ? '\u2588' : ' ');
    }
    Console.WriteLine();
}

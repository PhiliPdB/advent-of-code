
string[] instructions = File.ReadAllLines("./input.txt");

// Part 1
var scrambledPassword = new List<char> { 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h' };
foreach (string instruction in instructions) {
    string[] splitted = instruction.Split(' ');
    if (instruction.StartsWith("swap position")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[5]);

        (scrambledPassword[indexX], scrambledPassword[indexY]) = (scrambledPassword[indexY], scrambledPassword[indexX]);
    } else if (instruction.StartsWith("swap letter")) {
        int indexX = scrambledPassword.FindIndex(c => c == splitted[2][0]);
        int indexY = scrambledPassword.FindIndex(c => c == splitted[5][0]);

        (scrambledPassword[indexX], scrambledPassword[indexY]) = (scrambledPassword[indexY], scrambledPassword[indexX]);
    } else if (instruction.StartsWith("rotate left")) {
        int steps = int.Parse(splitted[2]);
        scrambledPassword = RotateLeft(scrambledPassword, steps).ToList();
    } else if (instruction.StartsWith("rotate right")) {
        int steps = int.Parse(splitted[2]);
        scrambledPassword = RotateRight(scrambledPassword, steps).ToList();
    } else if (instruction.StartsWith("rotate based")) {
        int steps = scrambledPassword.FindIndex(c => c == splitted[6][0]);
        int additional = steps >= 4 ? 2 : 1;

        scrambledPassword = RotateRight(scrambledPassword, (steps + additional) % scrambledPassword.Count).ToList();
    } else if (instruction.StartsWith("reverse")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[4]);
        scrambledPassword.Reverse(indexX, indexY - indexX + 1);
    } else if (instruction.StartsWith("move")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[5]);
        
        char c = scrambledPassword[indexX];
        scrambledPassword.RemoveAt(indexX);
        scrambledPassword.Insert(indexY, c);
    }
}
Console.WriteLine($"[Part 1] Scrambled password: {string.Concat(scrambledPassword)}");

// Part 2
var unScrambledPassword = new List<char> { 'f', 'b', 'g', 'd', 'c', 'e', 'a', 'h' };
foreach (string instruction in instructions.Reverse()) {
    string[] splitted = instruction.Split(' ');
    if (instruction.StartsWith("swap position")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[5]);

        (unScrambledPassword[indexX], unScrambledPassword[indexY]) = (unScrambledPassword[indexY], unScrambledPassword[indexX]);
    } else if (instruction.StartsWith("swap letter")) {
        int indexX = unScrambledPassword.FindIndex(c => c == splitted[2][0]);
        int indexY = unScrambledPassword.FindIndex(c => c == splitted[5][0]);

        (unScrambledPassword[indexX], unScrambledPassword[indexY]) = (unScrambledPassword[indexY], unScrambledPassword[indexX]);
    } else if (instruction.StartsWith("rotate left")) {
        int steps = int.Parse(splitted[2]);
        unScrambledPassword = RotateRight(unScrambledPassword, steps).ToList();
    } else if (instruction.StartsWith("rotate right")) {
        int steps = int.Parse(splitted[2]);
        unScrambledPassword = RotateLeft(unScrambledPassword, steps).ToList();
    } else if (instruction.StartsWith("rotate based")) {
        int currentIndex = unScrambledPassword.FindIndex(c => c == splitted[6][0]);
        var fromIndex = 0;
        while (fromIndex < unScrambledPassword.Count) {
            int additional = fromIndex >= 4 ? 2 : 1;
            if (((fromIndex + fromIndex + additional) % unScrambledPassword.Count) == currentIndex) {
                break;
            }

            fromIndex++;
        }

        int steps = currentIndex - fromIndex;
        if (steps < 0) {
            steps += unScrambledPassword.Count;
        }

        unScrambledPassword = RotateLeft(unScrambledPassword, steps).ToList();
    } else if (instruction.StartsWith("reverse")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[4]);
        unScrambledPassword.Reverse(indexX, indexY - indexX + 1);
    } else if (instruction.StartsWith("move")) {
        int indexX = int.Parse(splitted[2]);
        int indexY = int.Parse(splitted[5]);

        char c = unScrambledPassword[indexY];
        unScrambledPassword.RemoveAt(indexY);
        unScrambledPassword.Insert(indexX, c);
    }
}
Console.WriteLine($"[Part 2] Unscrambled password: {string.Concat(unScrambledPassword)}");




static IEnumerable<T> RotateLeft<T>(IEnumerable<T> e, int n) {
    return e.Skip(n).Concat(e.Take(n));
}

static IEnumerable<T> RotateRight<T>(IEnumerable<T> e, int n) {
    return RotateLeft(e.Reverse(), n).Reverse();
}


var strings = File.ReadAllLines("./input.txt");

var part1Score = strings
    .Sum(s => {
        var memStringChars = 0;
        
        var i = 1;
        while (i < s.Length - 1) {
            memStringChars++;

            if (s[i] == '\\') {
                if (s[i + 1] == 'x') {
                    i += 4;
                } else {
                    i += 2;
                }
            } else {
                i++;
            }
        }

        return s.Length - memStringChars;
    });
Console.WriteLine($"[Part 1] Score: {part1Score}");

var part2Score = strings
    .Sum(s => {
        var encodingChars = 2;
        foreach (char c in s) {
            if (c is '\\' or '"' or '\'') {
                encodingChars += 2;
            } else {
                encodingChars += 1;
            }
        }
        return encodingChars - s.Length;
    });
Console.WriteLine($"[Part 2] Score: {part2Score}");

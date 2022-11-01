using System.Text;

const string input = "1113122113";

static string GameRound(string s) {
    var sb = new StringBuilder();

    char lastChar = s[0];
    var charCount = 1;
    for (var i = 1; i < s.Length; i++) {
        if (s[i] != lastChar) {
            sb.Append(charCount);
            sb.Append(lastChar);

            lastChar = s[i];
            charCount = 1;
        } else {
            charCount++;
        }
    }
    sb.Append(charCount);
    sb.Append(lastChar);

    return sb.ToString();
}


string current = input;
for (var i = 0; i < 40; i++) {
    current = GameRound(current);
}
Console.WriteLine($"[Part 1] Resulting length: {current.Length}");

// NOTE: Already did 40 rounds
for (var i = 0; i < 10; i++) {
    current = GameRound(current);
}
Console.WriteLine($"[Part 2] Resulting length: {current.Length}");

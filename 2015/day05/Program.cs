
var strings = File.ReadAllLines("./input.txt");


static bool ContainsThreeVowels(string s) {
    return s.Count(c => c is 'a' or 'e' or 'i' or 'o' or 'u') >= 3;
}

static bool ContainsTwiceInRow(string s) {
    for (var i = 0; i < s.Length - 1; i++) {
        if (s[i] == s[i + 1]) {
            return true;
        }
    }

    return false;
}

static bool ContainsForbidden(string s) {
    return s.Contains("ab") || s.Contains("cd") || s.Contains("pq") || s.Contains("xy");
}

int niceStrings = strings.Count(s => ContainsThreeVowels(s) && ContainsTwiceInRow(s) && !ContainsForbidden(s));
Console.WriteLine($"[Part 1] Number of nice strings: {niceStrings}");


static bool ContainsTwoPairs(string s) {
    for (var i = 0; i < s.Length - 1; i++) {
        for (int j = i + 2; j < s.Length - 1; j++) {
            if (s[i] == s[j] && s[i + 1] == s[j + 1]) {
                return true;
            }
        }
    }

    return false;
}

static bool ConstainsRepeat(string s) {
    for (var i = 0; i < s.Length - 2; i++) {
        if (s[i] == s[i + 2]) {
            return true;
        }
    }

    return false;
}

int betterNiceStrings = strings.Count(s => ContainsTwoPairs(s) && ConstainsRepeat(s));
Console.WriteLine($"[Part 2] Number of nice strings: {betterNiceStrings}");


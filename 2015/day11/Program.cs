
const string input = "hepxcrrq";


static bool ContainsIncreasing(char[] p) {
    for (var i = 0; i < p.Length - 2; i++) {
        if (p[i + 1] == p[i] + 1 && p[i + 2] == p[i + 1] + 1) {
            return true;
        }
    }

    return false;
}

static bool ContainsInvalid(char[] p) {
    return p.Any(c => c is 'i' or 'o' or 'l');
}

static bool ContainsPairs(char[] p) {
    var pairs = 0;
    var index = 0;
    while (index < p.Length - 1) {
        if (p[index] == p[index + 1]) {
            pairs++;
            index += 2;
            continue;
        }

        index++;
    }
    return pairs >= 2;
}


char[] password = input.ToCharArray();

static void NextPassword(char[] password) {
    do {
        // Increase password
        for (int i = password.Length - 1; i >= 0; i--) {
            if (password[i] != 'z') {
                password[i]++;
                break;
            } else {
                password[i] = 'a';
            }
        }
    } while (!ContainsIncreasing(password) || ContainsInvalid(password) || !ContainsPairs(password));
}

NextPassword(password);
Console.WriteLine($"[Part 1] Found valid password: {string.Concat(password)}");

NextPassword(password);
Console.WriteLine($"[Part 2] Found valid password: {string.Concat(password)}");

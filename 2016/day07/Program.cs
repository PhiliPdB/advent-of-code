
IPv7[] input = File.ReadAllLines("./input.txt")
    .Select(line => new IPv7(line))
    .ToArray();

// Part 1
int supportsTLS = input.Count(ip => ip.SupportsTLS());
Console.WriteLine($"[Part 1] IPs supporting TLS: {supportsTLS}");

// Part 2
int supportsSSL = input.Count(ip => ip.SupportsSSL());
Console.WriteLine($"[Part 2] IPs supporting SSL: {supportsSSL}");


// Classes

class IPv7 {
    private List<string> supernets = new();
    private List<string> hypernets = new();

    public IPv7(string ipString) {
        var currentString = "";
        foreach (char c in ipString) {
            switch (c) {
                case '[':
                    this.supernets.Add(currentString);
                    currentString = "";
                    break;
                case ']':
                    this.hypernets.Add(currentString);
                    currentString = "";
                    break;
                default:
                    currentString += c;
                    break;
            }
        }
        this.supernets.Add(currentString);
    }

    public bool SupportsTLS() {
        return this.supernets.Any(HasABBA) && !this.hypernets.Any(HasABBA);
    }

    public bool SupportsSSL() {
        var babs = this.supernets
            .SelectMany(FindABAs)
            .Select(aba => $"{aba[1]}{aba[0]}{aba[1]}");

        return babs.Any(bab => this.hypernets.Any(h => h.Contains(bab)));
    }

    private static bool HasABBA(string s) {
        for (var i = 0; i < s.Length - 3; i++) {
            if (s[i] == s[i + 3] && s[i + 1] == s[i + 2] && s[i] != s[i + 1]) {
                return true;
            }
        }
        return false;
    }

    private static List<string> FindABAs(string s) {
        List<string> abas = new();
        for (var i = 0; i < s.Length - 2; i++) {
            if (s[i] == s[i + 2] && s[i] != s[i + 1]) {
                abas.Add(s[i..(i + 3)]);
            }
        }
        return abas;
    }
}

using System.Text;

var lines = File.ReadAllLines("./input.txt");
string initialMolecule = lines[^1];

var replacementRules = new Dictionary<string, List<string>>();
foreach (var line in lines[..^2]) {
    var split = line.Split(" => ");
    if (!replacementRules.ContainsKey(split[0])) {
        replacementRules[split[0]] = new List<string>();
    }

    replacementRules[split[0]].Add(split[1]);
}

HashSet<string> GetNextMolecules(string m) {
    var nextMolecules = new HashSet<string>();
    
    for (var i = 0; i < m.Length; i++) {
        if (i < m.Length - 1 && char.IsLower(m[i + 1])) {
            if (!replacementRules.ContainsKey(m[i..(i + 2)])) {
                i++;
                continue;
            }

            foreach (var rule in replacementRules[m[i..(i + 2)]]) {
                var sb = new StringBuilder(m);
                sb.Remove(i, 2);
                sb.Insert(i, rule);
                nextMolecules.Add(sb.ToString());
            }

            i++;
        } else {
            if (!replacementRules.ContainsKey(m[i].ToString())) {
                continue;
            }

            foreach (var rule in replacementRules[m[i].ToString()]) {
                var sb = new StringBuilder(m);
                sb.Remove(i, 1);
                sb.Insert(i, rule);
                nextMolecules.Add(sb.ToString());
            }
        }
    }

    return nextMolecules;
}

Console.WriteLine($"[Part 1] Distinct new molecules: {GetNextMolecules(initialMolecule).Count}");

// Part 2
// With the help of https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/?utm_source=share&utm_medium=web2x&context=3

var medicineLength = 0;

var ySymbols = 0;
var rnSymbols = 0;
var arSymbols = 0;

for (var i = 0; i < initialMolecule.Length - 1; i++) {
    if (initialMolecule[i] == 'Y') {
        ySymbols++;
    } else if (initialMolecule[i..(i + 2)] == "Rn") {
        rnSymbols++;
    } else if (initialMolecule[i..(i + 2)] == "Ar") {
        arSymbols++;
    }

    if (char.IsUpper(initialMolecule[i])) {
        medicineLength++;
    }
}
medicineLength++;

Console.WriteLine($"[Part 2] Medicine can be made in {medicineLength - rnSymbols - arSymbols - 2 * ySymbols - 1} steps");

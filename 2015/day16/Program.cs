
Aunt[] aunts = File.ReadAllLines("./input.txt")
    .Select(line => {
        var split = line.Split(' ');

        var aunt = new Aunt(int.Parse(split[1][..^1]));
        aunt.AddProperty(split[2][..^1], int.Parse(split[3][..^1]));
        aunt.AddProperty(split[4][..^1], int.Parse(split[5][..^1]));
        aunt.AddProperty(split[6][..^1], int.Parse(split[7]));

        return aunt;
    })
    .ToArray();


int FindAunt(Dictionary<string, int> properties) {
    foreach (Aunt aunt in aunts) {
        var foundAunt = true;
        foreach ((string name, int value) in aunt.properties) {
            foundAunt = foundAunt && properties[name] == value;
        }

        if (foundAunt) {
            return aunt.number;
        }
    }

    throw new Exception("Could not find aunt");
}

var properties = new Dictionary<string, int> {
    ["children"] = 3,
    ["cats"] = 7,
    ["samoyeds"] = 2,
    ["pomeranians"] = 3,
    ["akitas"] = 0,
    ["vizslas"] = 0,
    ["goldfish"] = 5,
    ["trees"] = 3,
    ["cars"] = 2,
    ["perfumes"] = 1,
};
Console.WriteLine($"[Part 1] Found Sue {FindAunt(properties)}");

int FindAuntRanges(Dictionary<string, int> properties) {
    foreach (Aunt aunt in aunts) {
        var foundAunt = true;
        foreach ((string name, int value) in aunt.properties) {
            foundAunt = name switch {
                "cats" or "trees"         => foundAunt && properties[name] < value,
                "pomerians" or "goldfish" => foundAunt && properties[name] > value,
                _                         => foundAunt && properties[name] == value
            };
        }

        if (foundAunt) {
            return aunt.number;
        }
    }

    throw new Exception("Could not find aunt");
}
Console.WriteLine($"[Part 2] Found Sue {FindAuntRanges(properties)}");


struct Aunt {
    public int number;
    public Dictionary<string, int> properties;

    public Aunt(int number) {
        this.number = number;
        this.properties = new Dictionary<string, int>(3);
    }

    public void AddProperty(string name, int value) {
        this.properties[name] = value;
    }
}

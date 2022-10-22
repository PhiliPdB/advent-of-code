using System.Diagnostics;

Dictionary<int, (To high, To low)> botRules = new();
Dictionary<int, List<int>> botValues = new();

// Read input
foreach (string line in File.ReadAllLines("./input.txt")) {
    string[] parts = line.Split(' ');
    switch (parts[0]) {
        case "value": {
            int bot = int.Parse(parts[5]);
            int value = int.Parse(parts[1]);
            
            if (botValues.ContainsKey(bot)) {
                botValues[bot].Add(value);
            } else {
                botValues[bot] = new List<int>(2) { value };
            }
            break;
        }
        case "bot": {
            int bot = int.Parse(parts[1]);
            var low = new To {
                type = parts[5] == "bot" ? Type.Bot : Type.Output,
                id = int.Parse(parts[6])
            };
            var high = new To {
                type = parts[10] == "bot" ? Type.Bot : Type.Output,
                id = int.Parse(parts[11])
            };
            botRules[bot] = (high, low);
            break;
        }
    }
}

// Let the bots do their work

int part1Answer = -1;
Dictionary<int, int> output = new();

Queue<int> queue = new();
queue.Enqueue(botValues.First(v => v.Value.Count == 2).Key);

while (queue.TryDequeue(out int bot)) {
    int high = botValues[bot].Max();
    int low = botValues[bot].Min();

    if (high == 61 && low == 17) {
        part1Answer = bot;
    }

    (To highRule, To lowRule) = botRules[bot];
    int? highResult = ExecuteRule(highRule, high, botValues, output);
    int? lowResult = ExecuteRule(lowRule, low, botValues, output);

    if (highResult != null) {
        queue.Enqueue((int) highResult);
    }
    if (lowResult != null) {
        queue.Enqueue((int) lowResult);
    }
}

Console.WriteLine($"[Part 1] Responsible bot: {part1Answer}");
Console.WriteLine($"[Part 2] Result: {output[0] * output[1] * output[2]}");


// Functions

static int? ExecuteRule(To rule, int value, Dictionary<int, List<int>> botValues, Dictionary<int, int> output) {
    switch (rule.type) {
        case Type.Bot:
            if (botValues.ContainsKey(rule.id)) {
                Debug.Assert(botValues[rule.id].Count == 1);
                botValues[rule.id].Add(value);
                return rule.id;
            } else {
                botValues[rule.id] = new List<int>(2) { value };
            }
            break;
        case Type.Output:
            output[rule.id] = value;
            break;
        default:
            throw new ArgumentOutOfRangeException();
    }

    return null;
}


// Classes

internal enum Type {
    Bot, Output,
}

internal struct To {
    public Type type;
    public int id;
}

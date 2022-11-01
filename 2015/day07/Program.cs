
var wires = File.ReadAllLines("./input.txt")
    .Select(line => {
        var parts = line.Split(" -> ");
        return (parts[1], parts[0]);
    })
    .ToDictionary(p => p.Item1, p => p.Item2);


var wireCache = new Dictionary<string, ushort>();
ushort GetValue(string wire) {
    if (wireCache.ContainsKey(wire)) {
        return wireCache[wire];
    }

    if (ushort.TryParse(wire, out ushort result)) {
        wireCache[wire] = result;
        return result;
    } else {
        var instruction = wires[wire].Split(' ');
        
        if (instruction.Length == 1) {
            return GetValue(instruction[0]);
        }
        
        ushort value = instruction[1] switch {
            "AND"    => (ushort) (GetValue(instruction[0]) & GetValue(instruction[2])),
            "OR"     => (ushort) (GetValue(instruction[0]) | GetValue(instruction[2])),
            "LSHIFT" => (ushort) (GetValue(instruction[0]) << short.Parse(instruction[2])),
            "RSHIFT" => (ushort) (GetValue(instruction[0]) >> short.Parse(instruction[2])),
            _        => (ushort) ~GetValue(instruction[1])
        };
        wireCache[wire] = value;
        return value;
    }
}

ushort aValue = GetValue("a");
Console.WriteLine($"[Part 1] Value of wire 'a': {aValue}");

wireCache.Clear();
wireCache["b"] = aValue;
Console.WriteLine($"[Part 2] Value of wire 'a': {GetValue("a")}");

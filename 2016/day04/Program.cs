
Room[] rooms = File.ReadAllLines("./input.txt")
    .Select(l => new Room(l))
    .ToArray();

// Part 1
int realRoomsSum = rooms
    .Where(r => r.ValidChecksum())
    .Sum(r => r.Id);
Console.WriteLine($"[Part 1] Real rooms sum: {realRoomsSum}");

// Part 2
int northPoleObjects = rooms.First(r => r.Decrypt() == "northpoleobjectstorage").Id;
Console.WriteLine($"[Part 2] North pole objects are stored at: {northPoleObjects}");


// Classes

internal class Room {
    public int Id { get; }

    private string name;
    private string checksum;

    public Room(string roomString) {
        string[] s = roomString.Split('[');
        this.checksum = s[1].Split(']')[0];

        string[] parts = s[0].Split('-');
        this.Id = int.Parse(parts[^1]);

        this.name = string.Concat(parts[..^1]);
    }

    public bool ValidChecksum() {
        Dictionary<char, int> frequencies = new();
        foreach (char c in this.name) {
            if (!frequencies.ContainsKey(c)) {
                frequencies[c] = 1;
            } else {
                ++frequencies[c];
            }
        }

        KeyValuePair<char, int>[] list = frequencies.ToArray();
        Array.Sort(list, (pair1, pair2) => pair1.Value == pair2.Value
            ? pair1.Key.CompareTo(pair2.Key)
            : pair2.Value.CompareTo(pair1.Value)
        );
        string calculatedChecksum = string.Concat(list.Select(kvp => kvp.Key))[..5];

        return this.checksum == calculatedChecksum;
    }

    public string Decrypt() {
        return string.Concat(
            this.name.Select(c => (char) ((c - 'a' + this.Id) % 26 + 'a'))
        );
    }
}

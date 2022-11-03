using System.Text;
using System.Text.RegularExpressions;

string json = File.ReadAllText("./input.txt");

var regex = new Regex(@"[^0-9-]+");

int part1Sum = regex.Replace(json, @",")
    .Split(',')
    .Select(s => string.IsNullOrEmpty(s) ? 0 : int.Parse(s))
    .Sum();
Console.WriteLine($"[Part 1] Total sum: {part1Sum}");

// Part 2

var jsonWithoutRedObjects = (string) json.Clone();

int index;
while ((index = jsonWithoutRedObjects.IndexOf(":\"red\"")) != -1) {
    int objectStart = index;
    var depth = 0;
    while (true) {
        if (jsonWithoutRedObjects[objectStart] == '}') {
            depth++;
        } else if (jsonWithoutRedObjects[objectStart] == '{') {
            if (depth == 0) {
                break;
            }
            depth--;
        }

        objectStart--;
    }

    int objectEnd = index;
    while (true) {
        if (jsonWithoutRedObjects[objectEnd] == '{') {
            depth++;
        } else if (jsonWithoutRedObjects[objectEnd] == '}') {
            if (depth == 0) {
                break;
            }
            depth--;
        }

        objectEnd++;
    }

    var sb = new StringBuilder(jsonWithoutRedObjects);
    sb.Remove(objectStart, objectEnd - objectStart + 1);
    jsonWithoutRedObjects = sb.ToString();
}


int part2Sum = regex.Replace(jsonWithoutRedObjects, @",")
    .Split(',')
    .Select(s => string.IsNullOrEmpty(s) ? 0 : int.Parse(s))
    .Sum();
Console.WriteLine($"[Part 2] Total sum: {part2Sum}");

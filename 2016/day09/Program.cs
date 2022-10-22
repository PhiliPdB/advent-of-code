
string input = File.ReadAllLines("./input.txt")[0];

long DecompressedLength(string s, bool recursive) {
    long decompressedLength = 0;
    var pos = 0;
    while (pos < s.Length) {
        if (s[pos] == '(') {
            int closingOffset = s[pos..].Select((c, i) => (c, i)).First(p => p.c == ')').i;
            string[] marker = s[(pos + 1)..(pos + closingOffset)].Split('x');
            
            int chars = int.Parse(marker[0]);
            int times = int.Parse(marker[1]);
            if (recursive) {
                int start = pos + closingOffset + 1;
                decompressedLength += DecompressedLength(s[start..(start + chars)], recursive) * times;
            } else {
                decompressedLength += chars * times;
            }

            pos += closingOffset + chars + 1;
        } else {
            decompressedLength++;
            pos++;
        }
    }

    return decompressedLength;
}

Console.WriteLine($"[Part 1] Decompressed length: {DecompressedLength(input, false)}");
Console.WriteLine($"[Part 2] Decompressed length: {DecompressedLength(input, true)}");

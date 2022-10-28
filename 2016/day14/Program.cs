using System.Security.Cryptography;
using System.Text;

const string salt = "jlmsuwbz";

using var md5Hash = MD5.Create();
string GetHash(string s, int stretchDepth = 0) {
    for (var i = 0; i < stretchDepth + 1; i++) {
        byte[] hashBytes = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(s));
        s = Convert.ToHexString(hashBytes).ToLower();
    }
    return s;
}

List<int> GetKeys(int stretchDepth) {
    var keys = new List<int>();
    var potentialKeys = new List<(int index, char c)>();

    var index = 0;
    while (keys.Count < 64) {
        string hash = GetHash(salt + index, stretchDepth);

        var foundPotential = false;
        char lastCharacter = hash[0];
        var consecutiveCharCount = 1;
        for (var i = 1; i < hash.Length; i++) {
            if (hash[i] == lastCharacter) {
                consecutiveCharCount++;

                if (consecutiveCharCount == 3 && !foundPotential) {
                    potentialKeys.Add((index, lastCharacter));
                    foundPotential = true;
                }

                if (consecutiveCharCount == 5) {
                    // Cleanup
                    potentialKeys.RemoveAll(k => index - k.index > 1000);

                    // Find key combinations
                    keys.AddRange(potentialKeys
                        .Where(k => k.index != index && k.c == lastCharacter)
                        .Select(k => k.index)
                    );
                    potentialKeys.RemoveAll(k => k.index != index && k.c == lastCharacter);
                }
            } else {
                lastCharacter = hash[i];
                consecutiveCharCount = 1;
            }
        }

        index++;
    }

    return keys;
}

Console.WriteLine($"[Part 1] Index for 64th key: {GetKeys(0)[63]}");
Console.WriteLine($"[Part 2] Index for 64th key: {GetKeys(2016)[63]}");

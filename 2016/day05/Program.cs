using System.Security.Cryptography;
using System.Text;

var input = "reyedfim";


var part1Password = "";
char[] part2Password = { 'z', 'z', 'z', 'z', 'z', 'z', 'z', 'z' };

using var md5Hash = MD5.Create();
var index = 0;
while (true) {
    byte[] hashBytes = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(input + index));
    string hash = BitConverter.ToString(hashBytes).Replace("-", string.Empty);
    
    if (hash[0] == '0' && hash[1] == '0' && hash[2] == '0' && hash[3] == '0' && hash[4] == '0') {
        if (part1Password.Length < 8) {
            part1Password += hash[5];
        }

        if (int.TryParse(hash[5].ToString(), out int pos) && pos < 8 && part2Password[pos] == 'z') {
            part2Password[pos] = hash[6];
        }

        if (part1Password.Length == 8 && !part2Password.Contains('z')) {
            break;
        }
    }

    ++index;
}

Console.WriteLine($"[Part 1] Password: {part1Password.ToLower()}");
Console.WriteLine($"[Part 2] Password: {string.Concat(part2Password).ToLower()}");

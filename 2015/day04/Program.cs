using System.Security.Cryptography;
using System.Text;


const string secret = "yzbqklnj";

using var md5Hash = MD5.Create();

int part1Answer = -1;
var number = 1;
while (true) {
    byte[] byteHash = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(secret + number));
    string s = Convert.ToHexString(byteHash);

    if (s.StartsWith("00000")) {
        if (part1Answer == -1) {
            part1Answer = number;
        }
        if (s.StartsWith("000000")) {
            break;
        }
    }
    number++;
}

Console.WriteLine($"[Part 1] Answer: {part1Answer}");
Console.WriteLine($"[Part 2] Answer: {number}");

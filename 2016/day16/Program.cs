using System.Text;

static string ExpandTo(string initialState, int diskSize) {
    var sb = new StringBuilder(diskSize);
    sb.Append(initialState);

    while (sb.Length < diskSize) {
        sb.Append('0');
        for (int i = sb.Length - 2; i >= 0; i--) {
            switch (sb[i]) {
                case '0':
                    sb.Append('1');
                    break;
                case '1':
                    sb.Append('0');
                    break;
                default:
                    throw new Exception("Invalid state");
            }
        }
    }

    return sb.ToString();
}

static string GetChecksum(string data) {
    // Largest power of 2 that is smaller than the data length
    int blockSize = data.Length & ~(data.Length - 1);

    var sb = new StringBuilder(data.Length / blockSize);
    for (var i = 0; i < data.Length; i += blockSize) {
        var ones = 0;
        foreach (char c in data[i..(i + blockSize)]) {
            if (c == '1') {
                ones++;
            }
        }

        if (ones % 2 == 0) {
            sb.Append('1');
        } else {
            sb.Append('0');
        }
    }
    
    return sb.ToString();
}

const string initialState = "01000100010010111";

// Part 1
const int part1DiskSize = 272;
string expanded = ExpandTo(initialState, part1DiskSize);
Console.WriteLine($"[Part 1] Checksum: {GetChecksum(expanded[..part1DiskSize])}");


// Part 2
const int part2DiskSize = 35651584;
expanded = ExpandTo(initialState, part2DiskSize);
Console.WriteLine($"[Part 2] Checksum: {GetChecksum(expanded[..part2DiskSize])}");


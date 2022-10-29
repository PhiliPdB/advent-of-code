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
    while (data.Length % 2 == 0) {
        var sb = new StringBuilder(data.Length / 2);
        for (var i = 0; i < data.Length; i += 2) {
            if (data[i] == data[i + 1]) {
                sb.Append('1');
            } else {
                sb.Append('0');
            }
        }

        data = sb.ToString();
    }

    return data;
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


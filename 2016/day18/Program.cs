using System.Text;

static int SafeTiles(int rows, string startingRow) {
    var safeTiles = 0;
    foreach (char c in startingRow) {
        if (c == '.') {
            safeTiles++;
        }
    }

    string currentRow = startingRow;
    var nextRow = new StringBuilder(startingRow.Length);
    for (var row = 1; row < rows; row++) {
        for (var i = 0; i < startingRow.Length; i++) {
            // Get relevant tile safety
            bool leftIsSafe = i <= 0 || currentRow[i - 1] == '.';
            bool centerIsSafe = currentRow[i] == '.';
            bool rightIsSafe = i >= startingRow.Length - 1 || currentRow[i + 1] == '.';

            // Determine safety
            if ((!leftIsSafe && !centerIsSafe && rightIsSafe)
                || (leftIsSafe && !centerIsSafe && !rightIsSafe)
                || (leftIsSafe && centerIsSafe && !rightIsSafe)
                || (!leftIsSafe && centerIsSafe && rightIsSafe)) {
                nextRow.Append('^');
            } else {
                nextRow.Append('.');
                safeTiles++;
            }
        }

        currentRow = nextRow.ToString();
        nextRow.Clear();
    }

    return safeTiles;
}

string firstRow = File.ReadAllText("./input.txt").TrimEnd();
Console.WriteLine($"[Part 1] Safe tiles: {SafeTiles(40, firstRow)}");
Console.WriteLine($"[Part 2] Safe tiles: {SafeTiles(400_000, firstRow)}");

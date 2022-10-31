
string input = File.ReadAllText("./input.txt");

var currentFloor = 0;
int basementPosition = -1;
for (var i = 0; i < input.Length; i++) {
    switch (input[i]) {
        case '(':
            currentFloor++;
            break;
        case ')':
            currentFloor--;
            break;
        default:
            throw new Exception("Unexpected character");
    }

    if (basementPosition < 0 && currentFloor < 0) {
        basementPosition = i + 1;
    } 
}
Console.WriteLine($"[Part 1] Reached floor: {currentFloor}");
Console.WriteLine($"[Part 2] Entered basement at: {basementPosition}");

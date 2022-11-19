
const int row = 2978;
const int column = 3083;

// Determine which code
var codeNumber = 1;
var step = 1;
for (var i = 1; i < row; i++) {
    codeNumber += step;
    step++;
}

step++;
for (var i = 1; i < column; i++) {
    codeNumber += step;
    step++;
}

// Generate code
long code = 20151125;
for (var i = 1; i < codeNumber; i++) {
    code = (code * 252533) % 33554393;
}

Console.WriteLine($"[Part 1] Code: {code}");

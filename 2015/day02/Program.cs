
var sizes = File.ReadAllLines("./input.txt")
    .Select(line => line.Split('x').Select(int.Parse).ToArray());

var toOrder = sizes
    .Sum(size => {
        int side1 = size[0] * size[1];
        int side2 = size[1] * size[2];
        int side3 = size[0] * size[2];

        return 2 * side1 + 2 * side2 + 2 * side3 + Math.Min(Math.Min(side1, side2), side3);
    });
Console.WriteLine($"[Part 1] Square feet of wrapping paper to order: {toOrder}");

var ribbonLength = sizes
    .Sum(size => {
        int biggest = size.Max();

        int side1;
        int side2;
        if (size[0] == biggest) {
            side1 = size[1];
            side2 = size[2];
        } else if (size[1] == biggest) {
            side1 = size[0];
            side2 = size[2];
        } else {
            side1 = size[0];
            side2 = size[1];
        }

        return side1 * 2 + side2 * 2 + size[0] * size[1] * size[2];
    });
Console.WriteLine($"[Part 2] Feet of ribbon to order: {ribbonLength}");

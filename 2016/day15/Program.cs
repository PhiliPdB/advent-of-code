
var discs = new List<Disc> {
    new Disc(13, 1),
    new Disc(19, 10),
    new Disc(3, 2),
    new Disc(7, 1),
    new Disc(5, 3),
    new Disc(17, 5),
};

static int FindButtonTime(List<Disc> discs) {
    // Use Chinese Remainder Theorem to find at which time to press the button

    int time = (-discs[0].startingPosition % discs[0].numberOfPositions) + discs[0].numberOfPositions - 1;
    int factor = discs[0].numberOfPositions;
    var index = 1;

    while (index < discs.Count) {
        time += factor;

        if (discs[index].CoversSlot(time + index + 1)) {
            factor *= discs[index].numberOfPositions;
            index++;
        }
    }

    return time;
}

// Part 1
Console.WriteLine($"[Part 1] Press button at t = {FindButtonTime(discs)}");
// Part 2
discs.Add(new Disc(11, 0));
Console.WriteLine($"[Part 2] Press button at t = {FindButtonTime(discs)}");


internal struct Disc {
    public int numberOfPositions;
    public int startingPosition;

    public Disc(int numberOfPositions, int startingPosition) {
        this.numberOfPositions = numberOfPositions;
        this.startingPosition = startingPosition;
    }

    public bool CoversSlot(int time) {
        int discPosition = (this.startingPosition + time) % this.numberOfPositions;
        return discPosition == 0;
    }
}

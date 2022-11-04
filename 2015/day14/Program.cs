
var reindeer = File.ReadAllLines("./input.txt")
    .Select(line => {
        var split = line.Split(' ');
        return new Reindeer(split[0], int.Parse(split[3]), int.Parse(split[6]), int.Parse(split[13]));
    })
    .ToArray();

Console.WriteLine($"[Part 1] Winning reindeer distance: {reindeer.Max(r => r.DistanceTraveled(2503))}");

var points = new int[reindeer.Length];
for (var time = 1; time <= 2503; time++) {
    var leadingReindeer = 0;
    var distance = 0;
    for (var i = 0; i < reindeer.Length; i++) {
        if (reindeer[i].DistanceTraveled(time) > distance) {
            leadingReindeer = i;
            distance = reindeer[i].DistanceTraveled(time);
        }
    }

    points[leadingReindeer]++;
}
Console.WriteLine($"[Part 2] Points of winning reindeer: {points.Max()}");

internal struct Reindeer {
    public string name;
    public int speed;
    public int flightTime;
    public int restTime;

    public Reindeer(string name, int speed, int flightTime, int restTime) {
        this.name = name;
        this.speed = speed;
        this.flightTime = flightTime;
        this.restTime = restTime;
    }

    public int DistanceTraveled(int time) {
        int distancePerBurst = this.speed * this.flightTime;
        int bursts = time / (this.flightTime + this.restTime);

        int mod = time % (this.flightTime + this.restTime);
        return distancePerBurst * bursts + Math.Min(mod, this.flightTime) * this.speed;
    }
}

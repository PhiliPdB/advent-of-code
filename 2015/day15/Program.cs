
var ingredients = new[] {
    new Ingredient("sugar", 3, 0, 0, -3, 2),
    new Ingredient("sprinkles", -3, 3, 0, 0, 9),
    new Ingredient("candy", -1, 0, 4, 0, 1),
    new Ingredient("chocolate", 0, 0, -2, 2, 8),
};

var bestScore = 0;
var bestPart2Score = 0;
for (var sugar = 0; sugar < 100; sugar++) {
    for (var sprinkles = 0; sprinkles < 100; sprinkles++) {
        for (var candy = 0; candy < 100; candy++) {
            int chocolate = 100 - sugar - sprinkles - candy;
            if (chocolate < 0) {
                continue;
            }

            var solution = new int[] { sugar, sprinkles, candy, chocolate };
            var capacityScore = solution.Zip(ingredients).Sum(v => v.First * v.Second.capacity);
            if (capacityScore <= 0) { continue; }

            var durabilityScore = solution.Zip(ingredients).Sum(v => v.First * v.Second.durability);
            if (durabilityScore <= 0) { continue; }

            var flavorScore = solution.Zip(ingredients).Sum(v => v.First * v.Second.flavor);
            if (flavorScore <= 0) { continue; }

            var textureScore = solution.Zip(ingredients).Sum(v => v.First * v.Second.texture);
            if (textureScore <= 0) { continue; }

            int score = capacityScore * durabilityScore * flavorScore * textureScore;
            if (score > bestScore) {
                bestScore = score;
            }

            var calories = solution.Zip(ingredients).Sum(v => v.First * v.Second.calories);
            if (calories == 500 && score > bestPart2Score) {
                bestPart2Score = score;
            }
        }
    }
}

Console.WriteLine($"[Part 1] Best score: {bestScore}");
Console.WriteLine($"[Part 2] Best score: {bestPart2Score}");


internal struct Ingredient {
    public string name;

    public int capacity;
    public int durability;
    public int flavor;
    public int texture;
    public int calories;

    public Ingredient(string name, int capacity, int durability, int flavor, int texture, int calories) {
        this.name = name;
        this.capacity = capacity;
        this.durability = durability;
        this.flavor = flavor;
        this.texture = texture;
        this.calories = calories;
    }
}

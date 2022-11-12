
const int input = 33_100_000;

static int FactorSum(int n) {
    var sum = 1 + n;

    for (var i = 2; i <= Math.Sqrt(n); i++) {
        if (n % i == 0) {
            sum += i;
            if (n / i != i) {
                sum += n / i;
            }
        }
    }

    return sum;
}

var currentHouse = 2;
while (FactorSum(currentHouse) < input / 10) {
    currentHouse += 2;
}
Console.WriteLine($"[Part 1] House: {currentHouse}");

// Part 2

var presents = new int[1_000_000];
for (var i = 1; i <= input; i++) {
    var visits = 0;
    for (int j = i; visits < 50 && j < presents.Length; j += i) {
        presents[j] += i;
        
        visits++;
    }
}

for (var i = 0; i < presents.Length; i++) {
    if (presents[i] >= input / 11) {
        Console.WriteLine($"[Part 2] House: {i}");
        break;
    }
}

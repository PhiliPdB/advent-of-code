
// Puzzle input
const int bossHitPoints = 103;
const int bossDamage = 9;
const int bossArmor = 2;


static bool IsPlayerWin(int playerHitPoints, int playerDamage, int playerArmor) {
    int playerHits = (int) Math.Ceiling((double) bossHitPoints / Math.Max(playerDamage - bossArmor, 1));
    int bossHits = (int) Math.Ceiling((double) playerHitPoints / Math.Max(bossDamage - playerArmor, 1));
    return playerHits <= bossHits;
}

var damageCost = new Dictionary<int, (int price, int rings)[]> {
     [4] = new[] {   (8, 0) },
     [5] = new[] {  (10, 0) },
     [6] = new[] {  (25, 0) },
     [7] = new[] {  (40, 0) },
     [8] = new[] {  (74, 0), (65, 1) },
     [9] = new[] {  (90, 1) },
    [10] = new[] { (124, 1), (115, 2) },
    [11] = new[] { (174, 1), (149, 2) },
    [12] = new[] { (199, 2) },
    [13] = new[] { (224, 2) },
};

var armorCost = new Dictionary<int, (int price, int rings)[]> {
     [0] = new[] {   (0, 0) },
     [1] = new[] {  (13, 0) },
     [2] = new[] {  (31, 0) },
     [3] = new[] {  (53, 0), (51, 1) },
     [4] = new[] {  (75, 0), (71, 1) },
     [5] = new[] { (102, 0), (93, 1), (91, 2) },
     [6] = new[] { (115, 1), (113, 2) },
     [7] = new[] { (142, 1), (135, 2) },
     [8] = new[] { (182, 1), (162, 2) },
     [9] = new[] { (202, 2) },
    [10] = new[] { (222, 2) },
};

var cheapest = int.MaxValue;
foreach ((int damage, (int price, int rings)[] dCosts) in damageCost) {
    foreach ((int armor, (int price, int rings)[] aCosts) in armorCost) {
        if (IsPlayerWin(100, damage, armor)) {
            foreach (var dCost in dCosts) {
                foreach (var aCost in aCosts) {
                    if (dCost.rings + aCost.rings <= 2 && dCost.price + aCost.price < cheapest) {
                        cheapest = dCost.price + aCost.price;
                    }
                }
            }
        }
    }
}
Console.WriteLine($"[Part 1] Cheapest win: {cheapest}");

// Part 2
var weapons = new (int cost, int damage)[] { (8, 4), (10, 5), (25, 6), (40, 7), (74, 8) };
var plates = new (int cost, int armor)[] { (0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5) };
var rings = new (int cost, int damage, int armor)[] { (0, 0, 0), (25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3) };

int MostExpensiveCombo(int damage, int armor) {
    return (
        from weapon in weapons
        from plate in plates
        from ring1 in rings
        from ring2 in rings
        where ring1 != ring2
        where weapon.damage + ring1.damage + ring2.damage == damage
        where plate.armor + ring1.armor + ring2.armor == armor
        select weapon.cost + plate.cost + ring1.cost + ring2.cost
    ).Prepend(0).Max();
}


var mostExpensive = 0;
for (var damage = 4; damage <= 13; damage++) {
    for (var armor = 0; armor <= 10; armor++) {
        if (!IsPlayerWin(100, damage, armor)) {
            var cost = MostExpensiveCombo(damage, armor);
            if (cost > mostExpensive) {
                mostExpensive = cost;
            }
        }
    }
}
Console.WriteLine($"[Part 2] Most expensive loss: {mostExpensive}");

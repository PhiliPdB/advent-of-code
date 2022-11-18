
const int bossHitPoints = 51;
const int bossDamage = 9;

const int playerHitPoints = 50;
const int playerMana = 500;

int MinManaSpent(bool hardMode = false) {
    var queue = new Queue<Status>();
    queue.Enqueue(new Status());

    while (queue.TryDequeue(out Status status)) {
        if (hardMode && status.playerTurn) {
            status.playerHealth--;
        }

        // Check win condition
        if (status.bossHealth <= 0) {
            return status.manaSpent;
        }

        if (status.playerHealth <= 0) {
            continue;
        }

        // Apply effects

        var hasShield = false;
        if (status.shieldEffectTurnsLeft > 0) {
            hasShield = true;
            status.shieldEffectTurnsLeft--;
        }

        if (status.poisonEffectTurnsLeft > 0) {
            status.bossHealth -= 3;
            status.poisonEffectTurnsLeft--;
        }

        if (status.rechargeEffectTurnsLeft > 0) {
            status.playerMana += 101;
            status.rechargeEffectTurnsLeft--;
        }

        if (!status.playerTurn) {
            int damageDealt = hasShield ? Math.Max(bossDamage - 7, 1) : bossDamage;

            queue.Enqueue(new Status(
                status.playerHealth - damageDealt, status.playerMana,
                status.bossHealth,
                status.shieldEffectTurnsLeft, status.poisonEffectTurnsLeft, status.rechargeEffectTurnsLeft,
                !status.playerTurn, status.manaSpent
            ));
            continue;
        }

        // Choose player action

        // Magic Missile
        if (status.playerMana >= 53) {
            queue.Enqueue(new Status(
                status.playerHealth, status.playerMana - 53,
                status.bossHealth - 4,
                status.shieldEffectTurnsLeft, status.poisonEffectTurnsLeft, status.rechargeEffectTurnsLeft,
                !status.playerTurn, status.manaSpent + 53
            ));
        }

        // Drain
        if (status.playerMana >= 73) {
            queue.Enqueue(new Status(
                status.playerHealth + 2, status.playerMana - 73,
                status.bossHealth - 2,
                status.shieldEffectTurnsLeft, status.poisonEffectTurnsLeft, status.rechargeEffectTurnsLeft,
                !status.playerTurn, status.manaSpent + 73
            ));
        }

        // Shield
        if (status.playerMana >= 113 && status.shieldEffectTurnsLeft == 0) {
            queue.Enqueue(new Status(
                status.playerHealth, status.playerMana - 113,
                status.bossHealth,
                6, status.poisonEffectTurnsLeft, status.rechargeEffectTurnsLeft,
                !status.playerTurn, status.manaSpent + 113
            ));
        }

        // Poison
        if (status.playerMana >= 173 && status.poisonEffectTurnsLeft == 0) {
            queue.Enqueue(new Status(
                status.playerHealth, status.playerMana - 173,
                status.bossHealth,
                status.shieldEffectTurnsLeft, 6, status.rechargeEffectTurnsLeft,
                !status.playerTurn, status.manaSpent + 173
            ));
        }

        // Recharge
        if (status.playerMana >= 229 && status.rechargeEffectTurnsLeft == 0) {
            queue.Enqueue(new Status(
                status.playerHealth, status.playerMana - 229,
                status.bossHealth,
                status.shieldEffectTurnsLeft, status.poisonEffectTurnsLeft, 5,
                !status.playerTurn, status.manaSpent + 229
            ));
        }
    }

    throw new Exception("Unreachable");
}

Console.WriteLine($"[Part 1] Killed boss using {MinManaSpent()} mana");
Console.WriteLine($"[Part 2] Killed boss using {MinManaSpent(true)} mana");


internal struct Status {
    public int playerHealth;
    public int playerMana;

    public int bossHealth;

    public int shieldEffectTurnsLeft;
    public int poisonEffectTurnsLeft;
    public int rechargeEffectTurnsLeft;

    public bool playerTurn = true;
    public int manaSpent;

    public Status() {
        this.playerHealth = 50;
        //this.playerHealth = 10;
        this.playerMana = 500;
        //this.playerMana = 250;

        this.bossHealth = 51;
        //this.bossHealth = 13;
    }

    public Status(
        int playerHealth, int playerMana,
        int bossHealth,
        int shieldEffectTurnsLeft, int poisonEffectTurnsLeft, int rechargeEffectTurnsLeft,
        bool playerTurn, int manaSpent
    ) {
        this.playerHealth = playerHealth;
        this.playerMana = playerMana;
        this.bossHealth = bossHealth;
        this.shieldEffectTurnsLeft = shieldEffectTurnsLeft;
        this.poisonEffectTurnsLeft = poisonEffectTurnsLeft;
        this.rechargeEffectTurnsLeft = rechargeEffectTurnsLeft;
        this.playerTurn = playerTurn;
        this.manaSpent = manaSpent;
    }
}

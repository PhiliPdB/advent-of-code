namespace day11;

internal enum Type {
    Chip, Generator
}

internal enum Rock {
    Curium,
    Elerium,
    Dilithium,
    Plutionium,
    Ruthenium,
    Strontium,
    Thulium,
}

internal struct Item: IEquatable<Item>, IComparable<Item> {
    public Type type;
    public Rock rock;

    public Item(Type type, Rock rock) {
        this.type = type;
        this.rock = rock;
    }

    public bool Equals(Item other) {
        return this.type == other.type && this.rock == other.rock;
    }

    public override bool Equals(object? obj) {
        return obj is Item other && this.Equals(other);
    }

    public override int GetHashCode() {
        return HashCode.Combine((int) this.type, this.rock);
    }

    public int CompareTo(Item other) {
        int typeComparison = this.type.CompareTo(other.type);
        if (typeComparison != 0) return typeComparison;
        
        return this.rock.CompareTo(other.rock);
    }
}

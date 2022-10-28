namespace day11;

internal struct Priority: IComparable<Priority> {
    public int moves;
    public int topFloorItems;

    public int CompareTo(Priority other) {
        int movesComparison = this.moves.CompareTo(other.moves);
        if (movesComparison != 0) {
            return movesComparison;
        }

        return other.topFloorItems.CompareTo(this.topFloorItems);
    }
}

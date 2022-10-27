
State initialState = new(new List<Item>[] {
    new List<Item>{ new(Type.Generator, Rock.Strontium), new(Type.Chip, Rock.Strontium), new(Type.Generator, Rock.Plutionium), new(Type.Chip, Rock.Plutionium) },
    new List<Item>{ new(Type.Generator, Rock.Thulium), new(Type.Generator, Rock.Ruthenium), new(Type.Chip, Rock.Ruthenium), new(Type.Generator, Rock.Curium), new(Type.Chip, Rock.Curium) },
    new List<Item>{ new(Type.Chip, Rock.Thulium) },
    new List<Item>{ },
});
initialState.SortAll();

static int MinimumElevatorMoves(State initialState) {
    int totalItems = initialState.floor.Sum(f => f.Count);

    // Perform Dijkstra

    var queue = new PriorityQueue<State, Priority>();
    var visited = new HashSet<State>();

    queue.Enqueue(initialState, new Priority { moves = 0, topFloorItems = 0 });
    visited.Add(initialState);

    while (queue.TryDequeue(out State state, out Priority p)) {
        // Stopping condition
        if (state.floor[3].Count == totalItems) {
            return p.moves;
        }

        // Generate next neighbours
        foreach (State newState in state.NextStates()) {
            if (visited.Add(newState)) {
                queue.Enqueue(newState, new Priority {
                    moves = p.moves + 1,
                    topFloorItems = newState.floor[3].Count,
                });
            }
        }
    }

    throw new Exception("Unreachable");
}

Console.WriteLine($"[Part 1] It took {MinimumElevatorMoves(initialState)} moves to bring everything to the fourth floor.");

initialState.floor[0].Add(new Item(Type.Generator, Rock.Elerium));
initialState.floor[0].Add(new Item(Type.Chip, Rock.Elerium));
initialState.floor[0].Add(new Item(Type.Generator, Rock.Dilithium));
initialState.floor[0].Add(new Item(Type.Chip, Rock.Dilithium));
initialState.SortFloor(0);
Console.WriteLine($"[Part 2] It took {MinimumElevatorMoves(initialState)} moves to bring everything to the fourth floor.");



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

internal class State: ICloneable, IEquatable<State> {
    public int elevatorLocation = 0;
    public List<Item>[] floor;

    private List<Item> CurrentFloorItems => this.floor[this.elevatorLocation];

    public State(List<Item>[] floor) {
        this.floor = floor;
    }

    public bool IsFeasibleFloor(int floor) {
        int numberOfRtgs = this.floor[floor].Count(i => i.type == Type.Generator);
        if (numberOfRtgs == 0) {
            return true;
        }

        foreach (Item item in this.floor[floor]) {
            if (item.type != Type.Chip) {
                continue;
            }
            
            // Find corresponding rtg
            int correspondingRtg = this.floor[floor].FindIndex(i => i.type == Type.Generator && i.rock == item.rock);
            if (correspondingRtg == -1) {
                return false;
            }
        }

        return true;
    }

    // Generate next states
    // - Take any two items on current floor
    // - Check if current floor left compatible
    // - Check if items chosen compatible
    // - Move to any other floor and check if compatible
    // - Repeat for only one item
    public List<State> NextStates() {
        var nextStates = new List<State>(3 * this.CurrentFloorItems.Count);
        
        for (var i = 0; i < this.CurrentFloorItems.Count; i++) {
            int[] newFloors = { this.elevatorLocation - 1, this.elevatorLocation + 1 };
            foreach (int newFloor in newFloors) {
                if (newFloor is < 0 or >= 4) {
                    continue;
                }

                // Only move this item
                var moveSingleNextState = (State) this.Clone();
                moveSingleNextState.floor[newFloor].Add(moveSingleNextState.CurrentFloorItems[i]);
                moveSingleNextState.CurrentFloorItems.RemoveAt(i);
                moveSingleNextState.elevatorLocation = newFloor;
                if (moveSingleNextState.IsFeasibleFloor(this.elevatorLocation) && moveSingleNextState.IsFeasibleFloor(newFloor)) {
                    moveSingleNextState.SortFloor(this.elevatorLocation);
                    moveSingleNextState.SortFloor(newFloor);

                    nextStates.Add(moveSingleNextState);
                }

                if (newFloor < this.elevatorLocation) {
                    continue;
                }

                // Move in combination with any other item
                for (var j = 0; j < i; j++) {
                    var moveDoubleNextState = (State) this.Clone();
                    moveDoubleNextState.floor[newFloor].Add(moveDoubleNextState.CurrentFloorItems[i]);
                    moveDoubleNextState.floor[newFloor].Add(moveDoubleNextState.CurrentFloorItems[j]);
                    moveDoubleNextState.CurrentFloorItems.RemoveAt(i);
                    moveDoubleNextState.CurrentFloorItems.RemoveAt(j);
                    moveDoubleNextState.elevatorLocation = newFloor;
                    if (moveDoubleNextState.IsFeasibleFloor(this.elevatorLocation) && moveDoubleNextState.IsFeasibleFloor(newFloor)) {
                        moveDoubleNextState.SortFloor(this.elevatorLocation);
                        moveDoubleNextState.SortFloor(newFloor);
                        
                        nextStates.Add(moveDoubleNextState);
                    }
                }
            }
        }

        return nextStates;
    }

    public void SortAll() {
        this.SortFloor(0);
        this.SortFloor(1);
        this.SortFloor(2);
        this.SortFloor(3);
    }

    public void SortFloor(int floor) {
        this.floor[floor].Sort();
    }

    public object Clone() {
        var clonedFloors = new List<Item>[4];
        for (var i = 0; i < 4; i++) {
            clonedFloors[i] = new List<Item>(this.floor[i].Count);
            foreach (Item item in this.floor[i]) {
                clonedFloors[i].Add(item);
            }
        }

        return new State(clonedFloors) {
            elevatorLocation = this.elevatorLocation
        };
    }

    public bool Equals(State? other) {
        if (ReferenceEquals(null, other)) return false;
        if (ReferenceEquals(this, other)) return true;
        return this.elevatorLocation == other.elevatorLocation
               && this.floor[0].SequenceEqual(other.floor[0])
               && this.floor[1].SequenceEqual(other.floor[1])
               && this.floor[2].SequenceEqual(other.floor[2])
               && this.floor[3].SequenceEqual(other.floor[3]);
    }

    public override bool Equals(object? obj) {
        if (ReferenceEquals(null, obj)) return false;
        if (ReferenceEquals(this, obj)) return true;
        if (obj.GetType() != this.GetType()) return false;
        return this.Equals((State) obj);
    }

    public override int GetHashCode() {
        int FloorHashCode(int floor) {
            return this.floor[floor].Aggregate(397, (x, y) => x * 31 + y.GetHashCode());
        }

        return HashCode.Combine(this.elevatorLocation, FloorHashCode(0), FloorHashCode(1), FloorHashCode(2), FloorHashCode(3));
    }
}

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

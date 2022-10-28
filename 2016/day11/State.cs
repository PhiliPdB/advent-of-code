namespace day11;

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

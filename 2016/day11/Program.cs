using day11;
using Type = day11.Type;


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

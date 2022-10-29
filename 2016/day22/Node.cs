namespace day22; 

public class Node {
    public int X { get; init; }
    public int Y { get; init; }

    public int Capacity { get; init; }
    public int Usage { get; private set; }

    public Node(int x, int y, int capacity, int usage) {
        this.X = x;
        this.Y = y;
        this.Capacity = capacity;
        this.Usage = usage;
    }
}

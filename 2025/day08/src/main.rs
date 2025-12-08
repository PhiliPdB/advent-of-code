use itertools::Itertools;
use union_find::{QuickFindUf, UnionBySize, UnionFind};

const PART1_CONNECTIONS: usize = 1000;

fn distance(pair: &[&(usize, (i64, i64, i64))]) -> f64 {
    let (x1, y1, z1) = pair[0].1;
    let (x2, y2, z2) = pair[1].1;

    f64::sqrt(((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64)
}

fn main() {
    let boxes: Vec<(_, (_, _, _))> = include_str!("../input.txt")
        .lines()
        .enumerate() // Use line number as node id
        .map(|(i, l)| {
            (
                i,
                l.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect();

    // Get all pairs and sort by distance
    let mut pairs: Vec<_> = boxes.iter().combinations(2).collect();
    pairs.sort_unstable_by(|a, b| {
        distance(a)
            .partial_cmp(&distance(b))
            .unwrap()
    });

    // Create union-find for quick merging
    let mut uf = QuickFindUf::<UnionBySize>::new(boxes.len());

    for p in pairs.iter().take(PART1_CONNECTIONS) {
        uf.union(p[0].0, p[1].0);
    }

    // Collect connected components by counting parent occurences
    let mut cc = vec![0; boxes.len()];
    for i in 0..boxes.len() {
        let root = uf.find(i);
        cc[root] += 1;
    }
    // Sort descending
    cc.sort_unstable_by(|a, b| b.cmp(a));

    println!("[Part 1] Size product: {}", cc[0] * cc[1] * cc[2]);

    // Part 2: Keep on connecting

    let mut p_iter = pairs.iter().skip(PART1_CONNECTIONS);
    loop {
        // Pairs will create a single fully connected component,
        // so program is finished before iterator is consumed.
        let p = p_iter.next().unwrap();
        uf.union(p[0].0, p[1].0);

        // Check if we have a single connected component
        let key = uf.find(0);
        if (1..boxes.len()).all(|i| uf.find(i) == key) {
            let c1 = p[0].1;
            let c2 = p[1].1;
            println!("[Part 2] Cable length required: {}", c1.0 * c2.0);
            break;
        }
    }
}

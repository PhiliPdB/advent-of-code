use itertools::Itertools;

const IMAGE_WIDTH: usize  = 25;
const IMAGE_HEIGHT: usize =  6;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Part 1

    let mut input_clone = input.clone();
    let best_layer = input_clone
        .chunks_mut(IMAGE_WIDTH * IMAGE_HEIGHT)
        .map(|layer| {
            layer.sort_unstable();

            layer.iter_mut().dedup_with_count().collect_vec()
        })
        .min_by(|layer1, layer2| {
            let layer1_zeros = (*layer1[0].1 == 0).then(|| layer1[0].0).unwrap_or(usize::MAX);
            let layer2_zeros = (*layer2[0].1 == 0).then(|| layer2[0].0).unwrap_or(usize::MAX);

            layer1_zeros.cmp(&layer2_zeros)
        })
        .unwrap();

    println!("Corruption check: {}", best_layer[1].0 * best_layer[2].0);

    // Part 2

    let mut final_image = vec![" "; IMAGE_WIDTH * IMAGE_HEIGHT];
    let layers: Vec<_> = input.chunks(IMAGE_WIDTH * IMAGE_HEIGHT).collect();

    for (i, p) in final_image.iter_mut().enumerate() {
        let mut layer_index = 0;
        while layers[layer_index][i] == 2 {
            layer_index += 1;
        }

        if layers[layer_index][i] == 1 {
            *p = "\u{2588}";
        }
    }

    println!("Final image:");
    for y in 0..IMAGE_HEIGHT {
        println!("{}", final_image[y * IMAGE_WIDTH..(y + 1) * IMAGE_WIDTH].concat())
    }
}

#[aoc(day8, part1)]
pub fn part1(enc_img: &str) -> usize {
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let num_layers = enc_img.len() / layer_size;

    let layers = separate_layers(enc_img, num_layers, layer_size);

    let layer_with_fewest_0 = layers
        .iter()
        .min_by(|layer, layer2| num_of(0, layer).cmp(&num_of(0, layer2)))
        .unwrap();

    num_of(1, layer_with_fewest_0) * num_of(2, layer_with_fewest_0)
}

fn separate_layers(enc_img: &str, num_layers: usize, layer_size: usize) -> Vec<Vec<usize>> {
    let mut layers = vec![];

    for n in 0..num_layers {
        let from = n * layer_size;
        let to = (n + 1) * layer_size;
        let layer: Vec<usize> = enc_img[from..to]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        layers.push(layer);
    }

    layers
}

fn num_of(num: usize, layer: &Vec<usize>) -> usize {
    layer.iter().filter(|&n| *n == num).count()
}

#[aoc(day8, part2)]
pub fn part2(enc_img: &str) -> usize {
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let num_layers = enc_img.len() / layer_size;

    let layers = separate_layers(enc_img, num_layers, layer_size);

    let img: Vec<usize> = collapse_layers(&layers, layer_size);

    print_img(&img, width, height);
    5
}

fn collapse_layers(layers: &Vec<Vec<usize>>, layer_size: usize) -> Vec<usize> {
    let mut img: Vec<usize> = vec![];

    for i in 0..layer_size {
        img.push(first_definite_color(layers, i, 0));
    }

    img
}

fn first_definite_color(layers: &Vec<Vec<usize>>, pos: usize, layer: usize) -> usize {
    if layers[layer][pos] == 2 {
        first_definite_color(layers, pos, layer + 1)
    } else {
        layers[layer][pos]
    }
}

fn print_img(img: &Vec<usize>, width: usize, height: usize) {
    for row in 0..height {
        let formatted_row: String = img[(row * width)..((row + 1) * width)]
            .iter()
            .map(|n| match n {
                0 => " ".to_string(),
                1 => "■".to_string(),
                other => format!("{}", other),
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{:?}", formatted_row);
    }
}

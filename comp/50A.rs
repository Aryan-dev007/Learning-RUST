fn main() {
    let mut first_input = String::new();
        std::io::stdin()
                    .read_line(&mut first_input)
                    .expect("Value not found");

    let inputs: Vec<usize> = first_input
                                .split_whitespace()
                                .map(|s| s.parse()
                                        .expect("Value not found"))
                                .collect();

    let m: usize = inputs[0];
    let n: usize = inputs[1];

    if (n < m) || (m >16) || (n < 1) || (m < 1) {
        return()
    }
    let boxes = n * m;
    println!("{}", boxes / 2);
}
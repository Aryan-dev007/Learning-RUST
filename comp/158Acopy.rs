fn main() {

    let mut first_input = String::new();
    std::io::stdin()
        .read_line(&mut first_input)
        .expect("Value Not found");

    let inputs: Vec<usize> = first_input
                            .split_whitespace()
                            .map(|s| s.parse().expect("Parse error"))
                            .collect();

    let num_contestants: usize = inputs[0];
    let k_positon: usize = inputs[1];

    if (k_positon > num_contestants) || (num_contestants > 50) {
        return ()
    }

    let mut second_input = String::new();
    std::io::stdin()
        .read_line(&mut second_input)
        .expect("Value not found");

    let score_input: Vec<usize> = second_input
                                        .split_whitespace()
                                        .map(|s| s.parse()
                                             .expect("parse error"))
                                        .collect();

    let count: usize = score_input.len();

    if (num_contestants != count) || (count > 100) {
        return ()
    }

    let mut i: usize = 0;
    for _i in 0..num_contestants-1 {
        if (score_input[i] > score_input[i+1]) || (score_input[i] == score_input[i+1]) {
            i += 1;
        }
        else {
            break;
        }
    }

    let mut final_count: usize = 0;
    let mut i: usize = 0;

    for _i in &score_input {
        if (score_input[k_positon -1] <= score_input[i]) && (score_input[i] > 0) {
            final_count = final_count + 1;
            i += 1;
        }
    }
    println!("{}", final_count);
}

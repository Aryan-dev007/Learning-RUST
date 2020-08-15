fn main() {
    let mut user_input = String::new();
        std::io::stdin()
                .read_line(&mut user_input)
                .expect("String not found");
    
    let user_inputs: Vec<&str> = user_input
                                    .split_whitespace()
                                    .collect();

    let inp1 = user_inputs[0];
    println!("{}", inp1);
}
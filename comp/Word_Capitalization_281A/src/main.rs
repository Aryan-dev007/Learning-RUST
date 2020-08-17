fn main() {
    let mut input = String::new();
    std::io::stdin()
            .read_line(&mut input)
            .expect("String not found");
    let result = first_upper_case(input);
    println!("{}", result);
}

fn first_upper_case(s: String) -> String { 
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
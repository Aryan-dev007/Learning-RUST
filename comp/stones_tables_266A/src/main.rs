fn main() {
    let mut input = String::new();
        std::io::stdin()
                .read_line(&mut input)
                .expect("Value not found");
    
    let input :usize = match input
                            .trim()
                            .parse() {
                                Ok(num) => num,
                                Err(_) => return,
                            };
            
    let mut colors = String::new();
        std::io::stdin()
                        .read_line(&mut colors)
                        .expect("Value not found");

    let length = colors.trim().len();

    if length != input {
        return ()
    }

    let mut count: usize = 0;
    let mut i: usize = 0;
    for _i in colors.chars() {
        if colors.chars().nth(i) == colors.chars().nth(i+1) {
            count += 1;
        }
            i += 1;
    }
    println!("{}", count);

}
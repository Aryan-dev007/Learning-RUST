
fn main() {
    let mut first_input = String::new();
    std::io::stdin()
            .read_line(&mut first_input)
            .expect("Error! Value not found");
    let n = first_input
                            .trim()
                            .parse::<usize>()
                            .unwrap();

    if (n < 1) || (n > 150) {
        return()
    }

    let mut count: i32 = 0;

    for _i in 0..n {

        let mut exp = String::from("");
            std::io::stdin()
                    .read_line(&mut exp)
                    .expect("String not found");

        match exp.trim() {
            "X++" => count += 1,
            "++X" => count += 1,
            "X--" => count -= 1,
            "--X" => count -= 1,
            _ => return()
        }
    }
    println!("{}", count);
}
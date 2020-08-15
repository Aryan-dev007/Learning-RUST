fn main() {
    let mut s = String::new();
    std::io::stdin()
                .read_line(&mut s)
                .expect("string not found");

    let word = first_word(&s);

    println!("{}", word);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
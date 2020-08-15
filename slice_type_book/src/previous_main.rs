fn main() {
    let mut s = String::new();
        std::io::stdin()
                .read_line(&mut s)
                .expect("String not found");

    let word = first_word(&s);

    println!("word_lenght: {}", word);
    s.clear();

    let s1 = String::from("there are you!");
    let slice1 = &s1[0..5];
    let slice2 = &s1[6..9];
    let slice3 = &s1[10..14];
    println!("slice 1 = {:?} slice2 = {:?} slice3 = {:?}", slice1, slice2, slice3);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn main() {
    let mut chances = String::new();
    std::io::stdin() 
                .read_line(&mut chances)
                .expect("Line not found");
    let chances = chances
                  .trim() 
                  .parse::<u32>()
                  .unwrap();

    let mut count = 0;

    while count < chances {

        count = count + 1;
        let mut words = String::new();
        std::io::stdin()
            .read_line(&mut words)
            .expect("Nothing entered");
        let length_word = words.chars().count() - 1;

        if length_word < 12 {
            print!("{}",words);
        }
        else {
            let vec_words: Vec<char> = words.chars().collect();
            let len1 = length_word-2;
            let len2 = length_word-1;
            print!("{}{}{}\n",vec_words[0],(len1),vec_words[len2]);
        }
        // else {
        //     let b: u8 = words.as_bytes()[0];
        //     let char_b: char = b as char;
            
        //     let c: u8 = words.as_bytes()[length_word-2];
        //     let char_c: char = c as char;

        //     print!("{}{}{}\n",char_b,(length_word-3),char_c);
        // }
}
}
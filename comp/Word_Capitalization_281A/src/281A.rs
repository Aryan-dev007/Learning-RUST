fn main() {

    let mut input = String::new();
    std::io::stdin()
            .read_line(&mut input)
            .expect("String not found");
    
    let mut str_vec: Vec<char> = input.chars().collect();
    if str_vec.len() > 1000 {
        return ()
    }
    let upper = str_vec[0].to_uppercase();
    str_vec.remove(0); 
    print!("{}", upper);
    for i in str_vec {
        print!("{}", i);
    }
}
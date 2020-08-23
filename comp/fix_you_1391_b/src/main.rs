fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");

    let mut t: usize = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    while t > 0 {
        solve();
        t -= 1;
    }

}

fn solve() {
    let input = multiple_input::<usize>().unwrap();
    let rows = input[0];
    let columns = input[1];

    let mut count_d = 0;
    let mut count_r = 0;
    for i in 0..rows {
        let mut elements = String::new();
        std::io::stdin().read_line(&mut elements).expect("Value not found");
        let elements = elements.replace("\n", "");
        let elements : Vec<char> = elements.chars().collect();
        // println!("{:?}", elements);

        if i < rows - 1 {
            if elements[columns -  1] == 'R' {count_r += 1;}
        }

if i == rows - 1 {
        for i in elements.iter() { 
        if i == &'D' {count_d += 1;} 
    }
}


}
        println!("{}", count_d + count_r);
}


fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin()
            .read_line(&mut s)
            .expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}
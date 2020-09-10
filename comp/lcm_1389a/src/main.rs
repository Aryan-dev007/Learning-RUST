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
    let inp = multiple_input::<i64>().unwrap();

    let l = inp[0];
    let r = inp[1];

    if l * 2 > r {
        println!("-1 -1");
    }
    else {
        println!("{} {}", l, l*2);
    }
}

fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}


fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");

    let mut t: usize = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let mut ans = 0;

    while t > 0 {
        // solve();
        let inp = multiple_input::<i64>().unwrap();
        let h = inp[0];
        let w  = inp[1];
        ans +=  h + w;
        t -= 1;
        let mut j = 0;
    }
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
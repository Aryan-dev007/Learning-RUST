
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
    let inp = multiple_input::<usize>().unwrap();
    let  n = inp[0];
    let  m = inp[1];

    if n % 2 == 0 || m % 2 == 0 {
        println!("{}", n*m/2);
    }
    else {
        println!("{}", (n*m+1)/2);
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
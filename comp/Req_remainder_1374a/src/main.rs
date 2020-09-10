fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");

    let mut t: usize = match t.trim().parse() {
        Ok(nu) => nu,
        Err(_) => return,
    };

    while t > 0 {
        solve();
        t -= 1;
    }
}

fn solve() {
    let arr = multiple_input::<usize>().unwrap();

    let x = arr[0];
    let y = arr[1];
    let n = arr[2];

    if n - n % x + y <= n {
        println!("{}", n - n%x + y);
    }
    else {
        println!("{}", n - n % x - (x - y));
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

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
    let inputs = multiple_input::<i64>().unwrap_or(vec![0]);
    let n = inputs[0];
    let k = inputs[1];

    if n < k {
        println!("{}", k-n);
    }

    else if n % 2 == k % 2  {
        println!("0");
    }

    else {
        println!("1");
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
fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("value not found");

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

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("value not found");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut a = multiple_input::<usize>().unwrap();
    let mut used: Vec<bool> = Vec::with_capacity(n);
    used.push(false);
    let mut p: Vec<usize> = Vec::new();

    for i in 0..n*2 {
        if !used[a[i] - 1] {
            used[a[i] - 1] = true;
            p.push(a[i]);
        }
    }

    for i in p {
        println!("{} ", i);
    }
    println!("");

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

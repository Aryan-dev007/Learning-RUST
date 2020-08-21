fn main() {

    let mut t = String::new();
    std::io::stdin()
        .read_line(&mut t)
        .expect("Value not found");

    let  t: usize = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    for _i in 0..t {
        solve();
    }
}

fn solve() {

    let mut n = String::new();
        std::io::stdin()
                .read_line(&mut n)
                .expect("Value not found");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let arr = multiple_input::<i64>().unwrap();
    let c = arr.len() - 1;
    if arr[0] + arr[1] <= arr[c] {
        println!("{} {} {}", 1, 2, c+1);
    }
    else {
        println!("-1");
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

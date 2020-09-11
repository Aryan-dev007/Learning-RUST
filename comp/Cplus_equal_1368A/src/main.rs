use std::mem::swap;
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
    let inp = multiple_input::<u64>().unwrap();
    let mut a = inp[0];
    let mut b = inp[1];
    let  n = inp[2];
    let mut count = 0;
    if a > b {
        swap(&mut a, &mut b);
    }

    while a <= n {
        a += b;
        swap(&mut a, &mut b);
        count += 1;
    }
    println!("{}", count - 1);
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
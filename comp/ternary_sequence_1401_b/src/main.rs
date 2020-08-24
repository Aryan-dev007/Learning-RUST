use std::cmp::min;
use std::io;
fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Value not found");

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
    let mut sum = 0;
    let mut m: i64;
    let mut a = multiple_input::<i64>().unwrap();
    let mut b = multiple_input::<i64>().unwrap();

    m = min(a[0], b[2]);
    a[0] -= m;
    b[2] -= m;


    m = min(a[1], b[0]);
    a[1] -= m;
    b[0] -= m;


    m = min(a[2], b[1]);
    a[2] -= m;
    b[1] -= m;

    sum += 2 * m;

    sum -= 2 * min(a[1], b[2]);

    println!("{}", sum);
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
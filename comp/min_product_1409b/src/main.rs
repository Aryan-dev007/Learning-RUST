use std::mem::swap;
use std::i64::MAX;
use std::cmp::min;
// use std::cmp::max;
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
    let arr = multiple_input::<i64>().unwrap();
    let mut a = arr[0];
    let mut b = arr[1];
    let mut t = arr[2];
    let mut k = arr[3];
    let n = arr[4];
    let mut ans= MAX;

    for _ in 0..2 {
        let da = min(n, a - t);
        let db = min(n - da, b-k);

        ans = min(ans, (a-da) * (b-db));
        // println!("before swap a = {} b = {}", a,b );
        // println!("before swap x = {} y = {}", t,k );

        swap(&mut a, &mut b);
        swap(&mut t , &mut k);

    }
    println!("{}", ans);

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


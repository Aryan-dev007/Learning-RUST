use std::cmp;
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
    // if l % r == 0 || r % l == 0 {
    //     println!("{} {}", l, r);
    // }
    // else if l % 2 == 0 && r % 2 != 0 ||  r % 2 == 0 && l % 2 != 0  || l == r {
    //     println!("-1 -1");
    // }
    // else {
    //     //   let m = cmp::max(l , r);
    //       let k = cmp::min(l, r);
    //       println!("{} {}", k, k*2 );
    // }
}

fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn hcf(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    hcf(b % a, a)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / hcf(a, b)
}

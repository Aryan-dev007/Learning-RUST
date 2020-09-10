use rand::Rng;
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
    // let inp = multiple_input::<i64>().unwrap();

loop { 
    let l = rand::thread_rng().gen_range(1, 10);
    let r = rand::thread_rng().gen_range(1, 10);

    if l * 2 > r {
        println!("-1 -1");
    }
    else {
        println!("{} {}", l, l*2);

        if lcm( l, l*2) > r {
            println!("l = {} r = {}", l, r);
            // println!("Lcm = {}, x = {}, y = {}", lcm(l, l*2), l, l*2);
            println!("wrong");
            std::process::exit(1);
        }
    }

 }
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
    return hcf(b % a, a)
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / hcf(a, b)
}

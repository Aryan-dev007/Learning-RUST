// use rand::Rng;
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
    // let n = rand::thread_rng().gen_range(1, 19);
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Value not found");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let eight = (n+3)/4;

    for _ in 0..n-eight {
        print!("9");
    }

    for _ in 0..eight {
        print!("8");
    }
    println!("");
}
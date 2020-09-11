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
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Value not found");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if n % 2 == 0 {
        println!("{}", n/2);
    }
    else {
        println!("{}", (n-1)/2);
    }

}


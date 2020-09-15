
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

    let mut n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut v: Vec<usize> = vec![];
    let mut multiplier = 1;

    while n > 0 {
    if n % 10 > 0 { 
            v.push ((n % 10) * multiplier);
        }
    n = n/10;
    multiplier *= 10;
    }

    println!("{}", v.len());
    for i in v {print! ("{} ", i);}
    println!("");
}

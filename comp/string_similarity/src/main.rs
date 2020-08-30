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

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    let s = s.replace("\n", "");
    let s: Vec<char> = s.chars().collect();

    for i in 0..n {
        print!("{}", s[n - 1]);
    }
    println!("");
}


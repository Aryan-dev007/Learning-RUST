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

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    // println!("{}", n / 2);

    if n <= 30 {
        println!("NO");
    }
    else {
        println!("YES");

        if n == 36 || n == 40 || n == 44 {
            println!("6 10 15 {}", n - 31);
        }
        else {
            println!("6 10 14 {}", n- 30);
        }
    }

}
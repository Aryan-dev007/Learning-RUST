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


fn solve () {

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Value not found");

    let mut n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut v: Vec<usize> = vec![];

    for i in 0..n {
        v.push(2<<i);
    }
    // println!("{:?}", v);
    println!("{}", v[n/2] - 2);

}
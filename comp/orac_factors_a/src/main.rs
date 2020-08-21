fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");
    let mut t:  i64 = match t.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };

    while t>0 {
        solve();
        t -= 1;
    }
}

fn solve() {
    let inputs = multiple_input::<i64>().unwrap();
    let n = inputs[0];
    let k = inputs[1];

    if  n % 2 == 0 {
        println!("{}", n+2*k);
    }
    else {
    let mut res = 0;
    for i in (3..=n).rev().step_by(2) {
        if n % i == 0{
            println!("i={}", i);
            res = i;
        }
    }
    println!("{}", n+res+2*(k-1));
}
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
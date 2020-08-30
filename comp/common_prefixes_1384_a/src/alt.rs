// use rand::Rng;
fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");

    let mut t: usize = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    while t > 0{
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

    let charset = String::from("abcdefghijklmnopqrstuvwxyz");
    // let num = String::from("1234567890");

    let mut inputs = multiple_input::<usize>().unwrap();
    // let inputs = inputs.insert(inputs[inputs.len() - 1], inputs.len() - 2);

    // println!("{:?}", inputs);


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
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

    let inp = multiple_input::<isize>().unwrap();
    let  a = inp[0];
    let  b = inp[1];

    // let mut count = 0;
    // if a < b{
    //     while a < b {
    //         a += 10;
    //         count += 1;
    //     }
    //     }
    // else {
    //     while a > b {
    //         a -= 10;
    //         count += 1;
    //     }
    // }

    // println!("{}",count);
        let diff = abs(a - b) + 9;

        println!("{}", diff/ 10);


}

fn abs(mut x: isize ) -> isize {
    if x < 0 { x = x * - 1; }
    else { x = x; }
    x
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

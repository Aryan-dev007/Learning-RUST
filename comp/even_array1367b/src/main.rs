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

    let  arr = multiple_input::<usize>().unwrap();
    let mut odd = 0;
    let mut even = 0;


    for (i,k) in arr.iter().enumerate() {
        if i%2 != k%2 {
            if i % 2 == 0 {
                even += 1;
            }
            else {
                odd += 1;
            }
        }
    }

    if odd != even {
        println!("-1");
    }
    else {
        println!("{}", odd);
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
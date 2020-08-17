fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Value not found");

    let mut t: i64 = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    if t > 100 || t < 1 {
        return ();
    }
    while t != 0 {
        solve();
        t -= 1;
    }
}

fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn solve() {
    let input = multiple_input::<i64>().unwrap();
    let n = input[0];
    let mut k = input[1];

    let mut arr = multiple_input::<i64>().unwrap();
    // arr.sort();

    if arr.len() != n as usize {
        return;
    }

    // let mut count = 0;
    if k % 2 == 1 {
        k = 1;
    } else {
        k = 2;
    }

    for _i in 0usize..k as usize {
    let big = arr.iter().max().unwrap().clone();

        for j in 0..arr.len() { 
            arr[j] = big - arr[j];
         }
        //  println!("{:?}, {}", arr, count);
        //  count += 1;
    }

    for j in 0..arr.len() {
        print!("{} ", arr[j]);
    }
    print!("\n");
}

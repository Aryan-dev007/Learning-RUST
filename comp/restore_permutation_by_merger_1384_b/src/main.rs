use std::collections::HashSet;
use std::hash::Hash;
fn main() {
    let mut t = String::new();
    std::io::stdin()
        .read_line(&mut t)
        .expect("value not found");

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
    std::io::stdin().read_line(&mut n).expect("value not found");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut inp = multiple_input::<usize>().unwrap();
    dedup(&mut inp);

    for i in inp {
        print!("{} ", i);
    }
    println!("");
}

fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

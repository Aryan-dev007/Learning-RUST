use std::cmp::max;
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


    let mut mx = 0;
    let  inputs = multiple_input::<usize>().unwrap();
    for i in inputs.clone() {
        mx = max(i, mx);
    }

    let mut st = String::from("");

    for _i in 0..mx+3 {
        st +="a";
    }

    println!("{}", st);

    for i in 0..n {
        let position = inputs[i];
        let st = st.replace("\n", "");
        let mut st: Vec<char> = st.chars().collect();
         if st[position] == 'y' { 
             st[position] = 'x';
        }
        else {
            st[position] = 'y';
        }
        let s: String = st.iter().map(ToString::to_string).collect();
        println!("{}", s);
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
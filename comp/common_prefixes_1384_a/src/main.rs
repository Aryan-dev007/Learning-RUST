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


    let  inputs = multiple_input::<usize>().unwrap();
    let mx = inputs.iter().max().unwrap();

    let mut st = String::from("");

    for _ in 0..mx+3 {
        st +="a";
    }

    println!("{}", st);

    for i in inputs  {
        // let pos = i;
        let st = st.replace("\n", "");
        let mut st: Vec<char> = st.chars().collect();
        //  if st[i] == 'y' {
        //      st[i] = 'x';
        //     //  println!("1");
        // }
        // else {
        //     st[i] = 'y';
        //     // println!("2");
        // }
        match st[i] {
            'y' => st[i] = 'x',
            // 'x' => st[i] = 'y',
            _ => st[i] = 'y',
        };
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
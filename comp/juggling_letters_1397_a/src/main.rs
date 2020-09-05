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
        let mut count = [0; 26];

    for _ in 0..n {

        let mut s = String::from("");
        std::io::stdin().read_line(&mut s).expect("Value not found");
        // let s = s.replace("\n", "");
        // let s: Vec<char> = s.chars().collect();

        for i in s.trim().as_bytes() {
            count[*i as usize - 97] += 1;
        }

        //  t = s.chars().nth(0).unwrap();
        // let count = s.matches(t).count();
        // final_count += count;
    }
    let mut flag = true;

    for i in 0..26 {
        if count[i] % n != 0 {
            flag = false;
            break;
        }
    }

    if flag {
        println!("YES");
    }
    else {
        println!("NO");
    }


}
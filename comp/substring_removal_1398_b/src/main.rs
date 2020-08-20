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

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    let s = s.replace("\n", "");
    let s: Vec<char> = s.chars().collect();
    // println!("{:?}", s);
    let mut ones : Vec<usize> = Vec::new();
    let mut i = 0;
    // for (mut k, i) in s.iter().enumerate(){
        while i < s.len() {
        if s[i] == '1' { 
            let mut j = i;
            while j+1 < s.len() && s[j+1] == '1' {j += 1}
            ones.push(j-i+1);
            i = j;
            i += 1;
    }
    else {
        i+=1;
    }
 }
        ones.sort();
        ones.reverse();
        let mut answer = 0;
        for (k,_) in ones.iter().enumerate() {
            if k % 2 == 0{
                answer += ones[k];
            }
        }
        println!("{}", answer);
}


// This question took me 2 days approx
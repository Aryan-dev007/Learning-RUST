fn main() {
    let mut position: isize = 0;
for j in 1..6 { 
    let mut input = String::new();
        std::io::stdin()
                    .read_line(&mut input)
                    .expect("Value not found");

    let input: Vec<isize> = input
                            .split_whitespace()
                            .map(|s| s.parse()
                                .expect("Parse Error"))
                            .collect();
    for i in 0..5 {
            if input[i] == 1 {
                let k = i as isize;
                let z = j as isize;
                position = abs(3 - z) + abs(3 - k - 1)
          }
 }
 }
 println!("{}", position);
}

fn abs(mut x: isize ) -> isize {
    if x < 0 { x = x * - 1; }
    else { x = x; }
    x
}
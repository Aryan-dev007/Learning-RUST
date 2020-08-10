// 4A
fn main() {
let mut melon = String::new();
std::io::stdin().read_line(&mut melon)
                .expect("Failed to read line");

let melon = melon
                .trim()
                .parse::<u32>().unwrap();
if melon % 2 == 0 {
    if melon != 2 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
else {
    println!("NO");
}
}

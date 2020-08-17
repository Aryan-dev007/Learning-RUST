fn main() {
    let mut test_cases_count = String::new();
    std::io::stdin()
        .read_line(&mut test_cases_count)
        .expect("Value not found");

    let mut test_cases_count: i64 = match test_cases_count.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    if test_cases_count > 100 || test_cases_count < 1 {
        return ();
    }
    while test_cases_count != 0 {
        let mut pass_length = String::new();
        std::io::stdin()
            .read_line(&mut pass_length)
            .expect("Value not found");

        let _pass_length: i64 = match pass_length.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        solve();
        test_cases_count -= 1;
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
        let mut password = multiple_input::<i64>().unwrap();

        password.sort(); // First we sort our array
        // println!("{:?}", password.sort());

        // 2 4 1 4 2 12 32
        // a sorted array 
        // 1 2 2 4 4 12 32
        // we can choose any index, so we add 12+32 then 12+32+4 then 12+32+4+4  so on 
        // no two number will be the same until the below condition is satisfied
        if password.first().unwrap() == password.last().unwrap() {  // comparing first element to last element of vector 
            // this condition can be satisfied as an array of 5 5 5 5 
            println!("{}", password.len());
            // println!("{:?}", password.last().unwrap());
        }
        else {
            println!("1");
        }

}
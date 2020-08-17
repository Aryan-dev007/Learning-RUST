fn main() {
    let mut number_of_test_cases = String::new();
    std::io::stdin()
                .read_line(&mut number_of_test_cases)
                .expect("Value not found");
    
    let number_of_test_cases: i64 = match number_of_test_cases 
                                                .trim()
                                                .parse() {
                                                    Ok(num) => num,
                                                    Err(_) => return,
                                                };
    if number_of_test_cases < 1 || number_of_test_cases > 1000 {
            return ()
    }
 
    for _i in 0i64..number_of_test_cases {
 
        let mut number_of_gifts = String::new();
            std::io::stdin()
                    .read_line(&mut number_of_gifts)
                    .expect("Value not found");
    let number_of_gifts: i64 = match number_of_gifts 
                                                .trim()
                                                .parse() {
                                                    Ok(num) => num,
                                                    Err(_) => return,
                                                };
    let candy = multiple_input::<i64>().unwrap();
    let min_value_candy = candy.iter().min().unwrap().clone();
 
    let oranges = multiple_input::<i64>().unwrap();
    let min_value_oranges = oranges.iter().min().unwrap().clone();
 
    if number_of_gifts as usize != candy.len() || candy.len() != oranges.len() {
         return ()
    } 
 
    let mut count : i64 = 0;
    for i in 0usize..number_of_gifts as usize {
        count += std::cmp::max(candy[i] - min_value_candy, oranges[i] - min_value_oranges);
    }
    println!("{}", count);
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
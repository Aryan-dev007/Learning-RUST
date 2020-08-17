fn main() {

    let mut number_of_test_cases = String::new();
    std::io::stdin()
            .read_line(&mut number_of_test_cases)
            .expect("Value not found");
    
    let number_of_test_cases: usize = match number_of_test_cases 
                                                .trim()
                                                .parse() {
                                                    Ok(num) => num,
                                                    Err(_) => return,
                                                };

    if number_of_test_cases < 1 || number_of_test_cases > 1000 {
            return ()
    }

    for _i in 0usize..number_of_test_cases {
     let mut count: usize = 0;
        let mut number_of_gifts = String::new();
            std::io::stdin()
                    .read_line(&mut number_of_gifts)
                    .expect("Value not found");
        
    let number_of_gifts: usize = match number_of_gifts 
                                                .trim()
                                                .parse() {
                                                    Ok(num) => num,
                                                    Err(_) => return,
                                                };

        let mut candy= String::new();
            std::io::stdin()
                    .read_line(&mut candy)
                    .expect("Value not found");
        let mut candy: Vec<usize> = candy.split_whitespace()
                                            .map(|s| s.parse() .expect("parse Error"))
                                            .collect();
        // println!("This is {:?}", candy);
        // println!("Candiogna length: {}", candy.len());
        let  min_value_candy   = candy.iter().min().unwrap().clone();
        // println!("minimum : {:?}", minValue);
        // if number_of_gifts != candy.len() 
        //     return ()

        let mut orange= String::new();
            std::io::stdin()
                    .read_line(&mut orange)
                    .expect("Value not found");
        let mut orange: Vec<usize> = orange.split_whitespace()
                                            .map(|s| s.parse() .expect("parse Error"))
                                            .collect();
        // println!("This is {:?}", orange);
        // println!("Candiogna length: {}", orange.len());
        let  min_value_orange   = orange.iter().min().unwrap().clone();

        if number_of_gifts != candy.len() || candy.len() != orange.len() {
            return ()
        }
            let mut z: usize = 0;
        loop {
            if z > number_of_gifts - 1 {
                break;
            }
            else if candy[z] > min_value_candy && orange[z] > min_value_orange {
                candy[z] -= 1;
                orange[z] -= 1;
                count += 1;
                // println!("Test");
            }
            else if candy[z] > min_value_candy {
                candy[z] -= 1;
                count += 1;
                // println!("test");
            }
            else if orange[z] > min_value_orange {
                orange[z] -= 1;
                count += 1;
                // println!("when");
            }
            else{
                z += 1;
                continue;
            }
        }
    // println!("ONe loop");
    println!("{}", count);
    }
    // println!("End");
}
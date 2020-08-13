fn main() {
    let mut number_of_inputs = String::new();
        std::io::stdin()
                .read_line(&mut number_of_inputs)
                .expect("Not an Integer");
    
    let number_of_inputs: u32 = match number_of_inputs
                                        .trim()
                                        .parse() {
                                            Ok(num) => num,
                                            Err(_)  => 0,
                                        };
    let mut count = 0;
    for _i in 0..number_of_inputs {
        
        let mut line = String::new();
        std::io::stdin()
                .read_line(&mut line)
                .expect("No number found");
        
        let inputs: Vec<&str> = line.split_whitespace().collect();
        let a: u32 = inputs[0].parse().unwrap();
        let b: u32 = inputs[1].parse().unwrap();
        let c: u32 = inputs[2].parse().unwrap();
        // Collecting values in Vec in string format "4k" (that input: Vec --- line)
        //Storing in Vec so indexing is possible 
        // parsing in u32 for making it's compnent acts like integer

        // println!("{} {} {}",a , b, c);

        if (a == 1 && b == 1) || (b == 1 && c == 1) || (a == 1 && c == 1) || (a == 1 && b == 1 && c == 1) {
            count += 1;
        }
    }
        println!("{}", count);
}
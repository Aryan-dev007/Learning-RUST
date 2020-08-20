fn main() {
    println!("---TEMPERATURE CONVERTER---");
    println!("From which to which ?
     1) Celsius to Farenheit - Enter 1
     2) Farenheit to Celsius - Enter 2");
loop {
     let mut val = String::new();
     std::io::stdin()
                    .read_line(&mut val)
                    .expect("Value not found");
    let val: u64 = match val
                            .trim()
                            .parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };

    println!("INPUT temperature value: ");

    let mut temp = String::new();
    std::io::stdin()
                .read_line(&mut temp)
                .expect("Value not found");
    let temp: u64 = match temp
                            .trim()
                            .parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };

    if val == 2 {
        println!("Converting Farenheit to Celsius");
        let res: u64 = (temp-32) * 5 / 9;
        println!("{} ", res);
        break;
    }

    else if   val == 1 {
        println!("Converting Celsius to Farenheit");
        let res: u64 = (temp * 9 / 5) + 32;
        println!("{} ", res);
        break;
    }

    else {
        println!("Wrong input BABY");
        continue;
    }
}
 }

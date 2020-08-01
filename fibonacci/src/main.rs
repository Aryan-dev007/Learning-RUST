fn main () {
    println!("__Fibonaaci SERIES__");
    println!("Enter the number: ",);

    let mut digit = String::new();
    std::io::stdin()
                .read_line(&mut digit)
                .expect("Digit Not found");
    let digit: u32 = match digit
                    .trim()
                    .parse() {
                        Ok(num) => num,
                        Err(_) => 1,
                    };
            fibo(digit);
}

fn fibo(x: u32) {
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut fib3;
    if x < 2 {
        println!("Fibonaaci Series does not exists");
    }
    else {
        println!("Fibonacci Series is: ");
        println!("{} \n{}", fib1, fib2);
        for _i in 3..x+1 {       //for i in range 3 to x.
            fib3 = fib1 + fib2;
            println!("{}", fib3);
            fib1 = fib2;
            fib2 = fib3;
        }
    }
}
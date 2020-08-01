use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number Game ");
    let secret_number = rand::thread_rng()
                            .gen_range(0, 101);
    /*
     *println!("You're secret number is {} ", secret_number);
    */

    loop {
        println!("Please input you're guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the number");

        let guess: u32 = match guess
                            .trim()
                            .parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };
        println!("You're guessed number is: ",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{
                println!("You win");
                break;
             }
        }
    }

}

fn main() {
    println!("\n");
    christmas_carol();
}

fn christmas_carol() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

    for (number_day, day) in days.iter().enumerate() {
        println!("For the {} day of christmas my true love sent to me ", day);

        for gifting_day in (1..(number_day + 1)).rev() {
            
            if gifting_day == 1 && number_day != 1 {
                println!("and");
            }

            if gifting_day == 1 {
                println!("a Patridge in a Pear Tree");
            }

            else if gifting_day == 2 {
                println!("Two Turtle Doves");
            }

            else if gifting_day == 3 {
                println!("Three French Hens");
            }

            else if gifting_day == 4 {
                println!("Four calling Birds");
            }

            else if gifting_day == 5 {
                println!("Five Golden Rings");
            }

            else if gifting_day == 6 {
                println!("Six Geese a Laying");
            }

            else if gifting_day == 7 {
                println!("Seven Swans a Swimming");
            }

            else if gifting_day == 8 {
                println!("Eight maid a Milking");
            }

            else if gifting_day == 9 {
                println!("Nine Ladies dancing");
            }

            else if gifting_day == 10 {
                println!("Ten lords leaping");
            }

            else if gifting_day == 11 {
                println!("Eleven Pipers Piping");
            }
            
            else if gifting_day == 12 {
                println!("12 Drummers Drumming");
            }
        }
        println!("\n");
    }
}
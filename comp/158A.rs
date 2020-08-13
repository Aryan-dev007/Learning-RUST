fn main() {

    let mut first_input = String::new();
    std::io::stdin()
            .read_line(&mut first_input)
            .expect("Value not found");

    let inputs: Vec<&str> = first_input.split_whitespace().collect(); 
    let num_contestant: i32 = inputs[0].parse().unwrap();
    let k_positon: i32 = inputs[1].parse().unwrap();
    // let num_contestant = num_contestant - 1;

    if (k_positon > num_contestant) || (num_contestant > 50) {
        return ()
    }

    // println!("n = {} k = {}", num_contestant, k_positon);

        let mut second_input = String::new();
        std::io::stdin()
                .read_line(&mut second_input)
                .expect("Value not found");

        let score_input: Vec<i32> = second_input.split_whitespace()
                                              .map(|s| s.parse()
                                              .expect("parse error"))
                                              .collect();

        let count: usize = score_input.len();
        let contestant_input_count: usize = inputs[0].parse().unwrap();
        // println!("score_input = {:?} count = {} ", score_input, count);
        // println!("contestant_input_count = {}", contestant_input_count);

        if  (contestant_input_count != count) || (count > 100) {
            return ()
        }

        // println!("Score input at 0: {:?}", score_input[0]);

        let mut i: usize = 0;
        for _i in 0..contestant_input_count-1 {
            if (score_input[i] > score_input[i+1]) || (score_input[i] == score_input[i+1]) {
                // println!("Check Sucess, {}", i); // don't need to compare the last element of the vector
                // so therefore  contestant_input_count - 1
                i = i + 1;
            }
            else {
                break;
            }
        }
        // let mut i: usize = 0;
        // while i < count-1 {
        //     if ( score_input[i] > score_input[i+1] ) || (score_input[i] == score_input[i+1]) {
        //         println!("check sucess, {}",i);
        //     }
        //     else {
        //         break;
        //     }
        //    i += 1;
        // }
        // let mut i: usize = 0;

        let mut final_count: usize = 0;
        let mut i: usize = 0;
        let k: usize = inputs[1].parse().unwrap();
        for _i in &score_input {
                if (score_input[k-1] <= score_input[i]) && (score_input[i] > 0) {
                final_count = final_count + 1;
                i += 1;
                }
            }
            println!("{}", final_count);
}
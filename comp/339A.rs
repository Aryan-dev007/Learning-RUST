fn main() { 
let mut ques = String::new();
    std::io::stdin()
            .read_line(&mut ques)
            .expect("Values not found");

let ques = ques.replace( "\n", "" );
let ques = ques.replace("+", " ");
let mut ques_each: Vec<usize> = ques.split_whitespace()
                                .map(|s| s.parse()
                                            .expect("value not found"))
                                .collect();
    // ques_each.sort();
// println!("{:?}", ques_each);
    sort(ques_each.as_mut_slice());
    // println!("{:?}", ques_each);

    let mut first = true;
    for i in ques_each {
        if !first {
            print!("+");
        }
        print!("{}",i);
        first = false;
    }
    println!("");

// let in1 = ques_each[0];
// let in2 = ques_each[1];
// let in3 = ques_each[2];
    // let test1 = in1 + in2;
// println!("{}+{}+{} ", in1, in2, in3);
}

fn sort(vec: &mut [usize]) {
    // let (mut i, mut j, mut small, mut temp): (usize, usize, usize, usize) = (0, 0, 0, 0);

    for i in 0..vec.len() - 1 {
        let mut small = i;
            for j in (i+1)..vec.len() {
                if vec[j] < vec[small] {
                    small = j;
                }
            }
            let temp = vec[i];
            vec[i] = vec[small];
            vec[small] = temp;
    } 
}
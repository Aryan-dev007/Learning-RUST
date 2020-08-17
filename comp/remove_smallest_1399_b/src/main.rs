fn main() {
    let mut number_of_test_cases = String::new();
    std::io::stdin()
        .read_line(&mut number_of_test_cases)
        .expect("Value not found");

    let number_of_test_cases: i64 = match number_of_test_cases.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    if number_of_test_cases < 1 || number_of_test_cases > 1000 {
        return ();
    }
    // let mut status = true;

    for _i in 0i64..number_of_test_cases {

        let mut number_of_elements = String::new();
        std::io::stdin()
            .read_line(&mut number_of_elements)
            .expect("Value not found");
        let  number_of_elements: i64 = match number_of_elements.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };

        let mut elements = multiple_input::<i64>().unwrap();
        // let min_value_candy = elements.iter().min().unwrap().clone();
        sort(elements.as_mut_slice());
        // println!("LLOo");
        let mut i = number_of_elements as usize - 1;
        // while i < number_of_elements as usize - 1
        if elements.len() != number_of_elements as usize {
            // println!("III");
            return ();
        }
        // println!("print i -> {}", i);
        while i > 0 {
            if abs(elements[i] - elements[i - 1]) == 1 {
                // println!("Removed");
                elements.remove(i);
                i -= 1;
            } else if elements[i] == elements[i - 1] {
                elements.remove(i - 1);
                // println!("fra");
                // println!("{:?}", elements);
                i -= 1;
            } else if abs(elements[i] - elements[i - 1]) > 1 {
                println!("NO");
                break;
            }
        }

        //     if status {
        //         println!("NO");
        //         break;
        //     }

        if elements.len() == 1 {
            println!("YES");
            // status = false;
        }
        // else if   abs(elements[i] - 2) > abs(elements[i-1])
    }
    return ();
}

fn multiple_input<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Value not found");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn abs(mut x: i64) -> i64 {
    if x < 0 {
        x = x * -1;
    } else {
        x = x;
    }
    x
}

fn sort(vec: &mut [i64]) {
    // let (mut i, mut j, mut small, mut temp): (usize, usize, usize, usize) = (0, 0, 0, 0);

    for i in 0..vec.len() - 1 {
        let mut small = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[small] {
                small = j;
            }
        }
        let temp = vec[i];
        vec[i] = vec[small];
        vec[small] = temp;
    }
}

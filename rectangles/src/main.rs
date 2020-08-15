struct Rectangle {
    width: usize,
    length: usize,
}

fn main() {
    let mut width = String::from("");
    std::io::stdin()
        .read_line(&mut width)
        .expect("Value not found");
    //   let width = width.trim().parse::<usize>().unwrap();

    let width: usize = match width.trim().parse() {
        Ok(num) => num,
        Err(_) => return (),
    };

    let mut length = String::from("");
    std::io::stdin()
        .read_line(&mut length)
        .expect("Value not found");
    //   let length = length.trim().parse::<usize>().unwrap();

    let length: usize = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => return (),
    };
    let rect1 = Rectangle { width, length };

    let areas = area(&rect1);

    println!("Value of area of the rectangle is: {:}", areas);
}

fn area(rectangle: &Rectangle) -> usize {
    rectangle.width * rectangle.length
}

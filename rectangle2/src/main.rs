#[derive(Debug)]

struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        self.radius * pi * self.radius
    }
}

fn main() {
    let mut width = String::new();
    std::io::stdin()
        .read_line(&mut width)
        .expect("Value not found");

    let width: usize = match width.trim().parse() {
        Ok(num) => num,
        Err(_) => return ,
    };

    let mut height = String::new();
    std::io::stdin()
        .read_line(&mut height)
        .expect("Value not found");

    let height: usize = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => return ,
    };
    let rect1 = Rectangle { width, height };

    let circle1 = Circle {
        radius: width as f64,
    };

    println!(
        "
        The area of circle is {:.3} square pixel ",
        circle1.area()
    );
    println!(
        "
        The area of rectangle is {} square pixels",
        rect1.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1: u32 = 30;
    // let height1: u32 = 50;

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectange is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);

    println!("rect1 has a non-zero width: {}", rect1.width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1: Rectangle = Rectangle::square(10);

    println!("square1 is {:?}", square1);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area2(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

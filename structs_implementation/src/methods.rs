#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Making main() function public
pub(crate) fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("The area of Rectangle is {:?}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

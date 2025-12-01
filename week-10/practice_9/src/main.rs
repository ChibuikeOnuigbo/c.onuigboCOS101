struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    println!("width is {}", small.width);
    println!("height is {}", small.height);
    println!("area of Rectangle is {}", small.area());
}

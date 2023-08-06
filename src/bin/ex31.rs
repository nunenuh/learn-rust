#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0 
    }
}

fn main(){
    let rect1: Rectangle = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixesls",
        rect1.area()
    );

    if rect1.width() {
        println!(
            "The rectangle has a nonzero witdh; it is {}",
            rect1.width
        )
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
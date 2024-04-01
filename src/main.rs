#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rect) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}


fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect{
        width:10,
        height:20
    };

    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());
    println!("Rect1 can hold Rect 2 {}", rect1.can_hold(&rect2));
    println!("Rect2 can hold Rect 1 {}", rect2.can_hold(&rect1));
}

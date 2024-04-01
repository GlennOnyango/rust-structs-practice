#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}",rect1);
    println!("The area of the rectangle is {}", area(&rect1));
}

fn area(rect: &Rect) -> i32 {
    rect.height * rect.width
}

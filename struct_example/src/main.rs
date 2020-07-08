// Find area of rectangle.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle = {:?}", rectangle);

    println!("Area of rectangle = {}", area(&rectangle));
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

use std::io;
#[derive(Debug)]

//      take number as input
//     let mut number = String::new();
//     println!("Enter the number : ");
//     io::stdin().read_line(&mut number).expect("Failed");
//     let num :i32 = number.trim().parse().expect("Enter the number");

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32),
}   

fn area(shape : Shape)->f32{
    match shape {
        Shape::Circle(radius)=> 3.14*radius*radius,
        Shape::Rectangle(l, b)=> l*b,
        Shape::Square(side)=>side*side,
    }

}

fn main() {
   let circle = Shape::Circle(23.0);
   let rect = Shape::Rectangle(23.0,33.0);
   let square = Shape::Square(11.0);

    println!("{}",area(circle));
}

mod circle;
mod rectangle;
mod area;

use circle::Circle;
use rectangle::Rectangle;
use area::Area;

fn main() {
    println!("Welcome to the area of circle and rectangle calculator");
    println!("Do you want to calculate the area of a circle or a rectangle? choose (c) or (r)");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");

    if choice.trim().to_lowercase() == "c" {
        println!("Enter the radius of the circle:");
        let mut radius = String::new();
        std::io::stdin().read_line(&mut radius).expect("Failed to read line");
        let radius: f64 = radius.trim().parse().expect("enter a number!");
    
        let circle = Circle::new(radius);
        println!("The area of the circle is: {}", (&circle as &dyn Area).area());
    } else if choice.trim().to_lowercase() == "r" {
        println!("Enter the length of the rectangle:");
        let mut length = String::new();
        std::io::stdin().read_line(&mut length).expect("Failed to read line");
        let length: f64 = length.trim().parse().expect(" enter a number");
    
        println!("Enter the width of the rectangle:");
        let mut width = String::new();
        std::io::stdin().read_line(&mut width).expect("Failed to read line");
        let width: f64 = width.trim().parse().expect(" enter a number");
    
        let rectangle = Rectangle::new(length, width);
        println!("The area of the rectangle is: {}", (&rectangle as &dyn Area).area());
    
    } else {
        println!("Choose C or R ");
    }
}
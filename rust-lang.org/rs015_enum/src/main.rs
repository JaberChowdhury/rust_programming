#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

// Added a function to use the data and calculate the area
fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(c) => std::f64::consts::PI * c.radius * c.radius,
        Shape::Rectangle(r) => r.width * r.height,
    }
}

fn increase_by_one(x: Option<f32>) -> Option<f32> {
    match x {
        None => None,
        Some(i) => Some(i + 1.0),
    }
}

fn main() {
    let my_circle = Shape::Circle(Circle { radius: 5.0 });
    let my_rect = Shape::Rectangle(Rectangle {
        width: 10.0,
        height: 20.0,
    });

    dbg!(increase_by_one(Some(12.21)));
    dbg!(increase_by_one(None));
    println!("Shape 1: {:?}", my_circle);
    println!("Area of circle: {:.2}", calculate_area(&my_circle));

    println!("Shape 2: {:?}", my_rect);
    println!("Area of rectangle: {:.2}", calculate_area(&my_rect));
}

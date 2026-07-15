// use std::f32::consts::PI;

// // #![no_std]
// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    // name: String,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// struct Circle {
//     radius: f32,
//     stroke: f32,
// }

// impl Circle {
//     fn area(self) -> f32 {
//         let area = 2 as f32 * PI * (self.radius + self.stroke);
//         area
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//         name: String::from("Good rect"),
//     };
//     let c1 = Circle {
//         radius: 12.21,
//         stroke: 3.2,
//     };

//     c1.area();

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

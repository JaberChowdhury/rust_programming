// // An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// // 1. A unit struct
// struct Unit;

// // 2. A tuple struct
// struct Pair(i32, f32);

// // 3. A struct with two fields
// struct Point {
//     x: f32,
//     y: f32,
// }
// #[derive(Debug)]
// struct Ipoint {
//     x: i32,
//     y: i32,
// }

// // Structs can be reused as fields of another struct
// struct Rectangle {
//     // A rectangle can be specified by where the top left and bottom right
//     // corners are in space.
//     top_left: Point,
//     bottom_right: Point,
// }

// #[derive(Debug)]
// struct Squarebox {
//     top_left: Ipoint,
//     top_right: Ipoint,
//     bottom_left: Ipoint,
//     bottom_right: Ipoint,
// }

// fn main() {
//     // Create struct with field init shorthand
//     let name = String::from("Peter");
//     let age = 27;
//     let peter = Person { name, age };
//     let mut all_boxes: Vec<Squarebox> = vec![];

//     for i in 1..10 {
//         all_boxes.push(Squarebox {
//             top_left: Ipoint { x: i * 2, y: i * 3 },
//             top_right: Ipoint { x: i * 3, y: i * 4 },
//             bottom_left: Ipoint { x: i * 1, y: i * 1 },
//             bottom_right: Ipoint { x: i * 4, y: i * 1 },
//         });
//     }
//     // println!("{:#?}", all_boxes);
//     dbg!(all_boxes);
//     // Print debug struct
//     println!("{:?}", peter);

//     // Instantiate a `Point`
//     let point = Point { x: 5.2, y: 0.4 };
//     let another_point = Point { x: 10.3, y: 0.2 };

//     // Access the fields of the point
//     println!("point coordinates: ({}, {})", point.x, point.y);

//     // Make a new point by using struct update syntax to use the fields of our
//     // other one
//     let bottom_right = Point {
//         x: 10.3,
//         ..another_point
//     };

//     // `bottom_right.y` will be the same as `another_point.y` because we used that field
//     // from `another_point`
//     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

//     // Destructure the point using a `let` binding
//     let Point {
//         x: left_edge,
//         y: top_edge,
//     } = point;

//     let _rectangle = Rectangle {
//         // struct instantiation is an expression too
//         top_left: Point {
//             x: left_edge,
//             y: top_edge,
//         },
//         bottom_right: bottom_right,
//     };

//     // Instantiate a unit struct
//     let _unit = Unit;

//     // Instantiate a tuple struct
//     let pair = Pair(1, 0.1);

//     // Access the fields of a tuple struct
//     println!("pair contains {:?} and {:?}", pair.0, pair.1);

//     // Destructure a tuple struct
//     let Pair(integer, decimal) = pair;

//     println!("pair contains {:?} and {:?}", integer, decimal);
// }
#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}
struct Rect(Point, Point, Point, Point);

fn calculate_area(item: &Rect) -> f32 {
    let trapezoid_1 = (item.0.x * item.1.y) - (item.1.x * item.0.y);
    let trapezoid_2 = (item.1.x * item.2.y) - (item.2.x * item.1.y);
    let trapezoid_3 = (item.2.x * item.3.y) - (item.3.x * item.2.y);
    let trapezoid_4 = (item.3.x * item.0.y) - (item.0.x * item.3.y);

    let sum = (trapezoid_1 + trapezoid_2 + trapezoid_3 + trapezoid_4) as f32;
    (sum.abs()) / 2.0
}

fn main() {
    let mut particles: Vec<Rect> = vec![];

    for i in 1..22 {
        particles.push(Rect(
            Point { x: i * 2, y: i * 3 },
            Point { x: i * 1, y: i * 2 },
            Point { x: i * 2, y: i * 3 },
            Point { x: i * 1, y: i * 2 },
        ));
    }

    // Loop to calculate and display the area of each shape
    for (index, p) in particles.iter().enumerate() {
        println!("Shape {}: Area = {}", index + 1, calculate_area(p));
    }
}

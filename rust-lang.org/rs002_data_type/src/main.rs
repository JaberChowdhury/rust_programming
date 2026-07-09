use std::vec;

fn main() {
    // let a: i8 = 123;
    // let a1: i16 = 1223;
    // let a2: i32 = 1233;
    // let a3: i64 = 123321231;
    // let dynamic_intiger: isize = 123321231;
    // println!("a = {a}");
    // println!("a1 = {a1}");
    // println!("a2 = {a2}");
    // println!("a3 = {a3}");
    // println!("dynamic intiger = {dynamic_intiger}");
    // let a: i32 = -42;
    // let b: u8 = 255;
    // let c = 98_222; // inferred i32
    // let hex = 0xff;

    // println!("{a} {b} {c} {hex}");

    // let floating_value = 3.141325432532535453267;
    // println!("{floating_value}");

    // let is_active: bool = true;
    // println!("{is_active}");

    // let key_pressed = 'A';
    // let letter = 'a';
    // let emoji = '😻';
    // let unicode = 'Ω';
    // println!("{letter}");
    // println!("{emoji}");
    // println!("{key_pressed}");
    // println!("{unicode}");

    // // tuple
    // let my_tuple: (i32, f32, bool) = (12312, 3.1416, true);
    // let (x, y, z) = my_tuple;
    // println!("{x} {y} {z}");

    // // array
    // let array: [i32; 10] = [12, 23, 34, 45, 56, 67, 78, 89, 90, 21];
    // for num in array.iter() {
    //     println!("{}", num * num / num + num - num % num);
    // }

    // // string
    // let my_string: String = String::from("value");
    // println!("{my_string}");

    // let complex_example: [(i32, f32, bool, String); 3] = [
    //     (12, 12.34, true, String::from("value")),
    //     (12, 12.34, true, String::from("value")),
    //     (12, 12.34, true, String::from("value")),
    // ];

    // for data in complex_example.iter() {
    //     println!("{}", data.3);
    // }

    // type Circle = (f32, f32);
    // let circle: [Circle; 2] = [(12.21, 233.32), (32.21, 233.32)];
    // for c in circle.iter() {
    //     println!("Radius of circle is {}", c.0);
    // }

    // struct Box<T, U, V> {
    //     width: T,
    //     height: U,
    //     color: V,
    // }
    // let red_box = Box {
    //     width: 1234,
    //     height: 2133,
    //     color: "red",
    // };

    // println!("color {}", red_box.color);
    // println!("width {}", red_box.width);
    // println!("height {}", red_box.height);

    // Growable array
    let mut dynamic_array: Vec<f32> = Vec::new();
    dynamic_array.push(1232.213123);
    for d in dynamic_array.iter() {
        println!("{d}");
    }
    let d_ar = vec![12, 2132, 32];
    for d in d_ar.iter() {
        println!("{d}");
    }
}

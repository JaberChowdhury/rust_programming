fn main() {
    print_hello();
    dbg!(add(21.12, 32.23));
    dbg!(sub(21.12, 32.23));
    dbg!(divide(21.12, 32.23));
    dbg!(power(21.12, 32.23));

    let float_base: f64 = 3.14;
    let int_exp: i32 = 2;
    let float_exp: f64 = 2.5;

    let res_i = float_base.powi(int_exp); // 3.14 ^ 2
    let res_f = float_base.powf(float_exp); // 3.14 ^ 2.5

    println!("Powi Result: {}", res_i);
    println!("Powf Result: {}", res_f);
}

fn print_hello() {
    println!("Hello, world!");
}

fn add(x: f32, y: f32) -> f32 {
    x + y
}
fn sub(x: f32, y: f32) -> f32 {
    x - y
}
fn divide(x: f32, y: f32) -> f32 {
    x / y
}
fn power(x: f32, y: f32) -> f32 {
    x.powf(y)
}

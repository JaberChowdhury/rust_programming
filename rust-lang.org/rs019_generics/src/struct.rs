#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_var = Point { x: 5, y: 10 };
    let float_var = Point { x: 1.0, y: 4.0 };
    dbg!(integer_var, float_var);
}

struct Rectangle {
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32),
}
impl Rectangle {
    fn area(dimension: (u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }

    fn distance(x: (f32, f32), y: (f32, f32)) -> f32 {
        let s1 = x.0 - y.0;
        let s2 = x.1 - y.1;
        s1.powf(2.0) * s2.powf(2.0)
    }
}
fn main() {
    let rect1 = (12, 21);
    let area = area(rect1);

    let p1 = (12.21, -23.32);
    let p2 = (23.43, -34.56);
    let d = distance(p1, p2);
    dbg!(area, d);
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn distance(x: (f32, f32), y: (f32, f32)) -> f32 {
    let s1 = x.0 - y.0;
    let s2 = x.1 - y.1;
    s1.powf(2.0) * s2.powf(2.0)
}

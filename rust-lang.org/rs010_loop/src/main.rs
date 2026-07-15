fn main() {
    let mut x = 0;
    let my_loop = loop {
        x += 1;
        dbg!(x);
        if x >= 10 {
            break;
        }
    };
    dbg!(my_loop);
    println!("Hello, world!");
}

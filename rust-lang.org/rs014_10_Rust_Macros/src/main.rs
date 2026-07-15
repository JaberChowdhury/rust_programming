fn main() {
    let data: &str = include_str!("data.json");
    dbg!(data.replace('\n', " ").replace('\\', " "));

    let icon: &[u8] = include_bytes!("image.png");
    println!("Icon size is {} byte", icon.len());
    println!("Icon size is {} KiloByte", icon.len() / 1024);
    println!(
        "Icon size is {} GigaByte",
        icon.len() as f32 / (1024 * 1024) as f32
    );
    println!("Hello, world!");
}

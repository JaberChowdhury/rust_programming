fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // dbg!("{}", s2);
    // let v = vec![1, 2, 3, 4, 21, 5];
    // let sum: i32 = v.iter().sum();

    // let mut x = "Hellow world";
    // dbg!(x);
    // x = "Hello world";
    // x.split("");
    // dbg!(x);
    // dbg!(sum.to_string());
    // let mut one = 123;
    // let mut two = 123;
    // (one, two) = (two, one);
    // dbg!(one, two);

    // let mut counter = 0;
    // dbg!(counter);
    // counter = 1; // reassign
    // counter += 10; // compound assign
    // // counter++;        // Rust has NO ++ or -- operator
    // counter += 1; // use this instead
    // dbg!(counter);
    let x = 10;

    // Immutable reference
    let r = &x;
    println!("{r}"); // 10

    // Mutable reference
    let mut y = 10;
    let r_mut = &mut y;
    *r_mut = 99;
    println!("{y}"); // 99
}

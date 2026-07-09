fn main() {
    let a = String::from("value");
    // let b = a;
    let b = a.clone();
    dbg!(a, b);
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow, s1 still valid after
    dbg!(len);
    dbg!(len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the data, so nothing is dropped

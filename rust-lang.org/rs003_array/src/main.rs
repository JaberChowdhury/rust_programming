fn main() {
    struct Box<X, Y, Z> {
        width: X,
        height: Y,
        depth: Z,
    }

    struct ReturnType {
        value: String,
        count: i32,
    }

    let mut box_collection: [Box<i32, f32, i32>; 3] = [
        Box {
            width: 12,
            height: 32.12,
            depth: 123,
        },
        Box {
            width: 12,
            height: 32.12,
            depth: 123,
        },
        Box {
            width: 12,
            height: 32.12,
            depth: 123,
        },
    ];

    let mut result_collection: [ReturnType; 2] = [
        ReturnType {
            value: String::from("value"),
            count: 12,
        },
        ReturnType {
            value: String::from("value2"),
            count: 12,
        },
    ];

    // Use the variables so there are no warnings.
    box_collection[0].width = 20;
    result_collection[0].count += 1;

    println!(
        "Box width: {}, Result: {} ({})",
        box_collection[0].width, result_collection[0].value, result_collection[0].count
    );
    println!(
        "width: {}, height: {}, depth: {}",
        box_collection[0].width, box_collection[0].height, box_collection[0].depth
    );

    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // Iterate by value
    for val in arr {
        println!("{val}");
    }

    // Iterate with index
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{i}] = {val}");
    }
}

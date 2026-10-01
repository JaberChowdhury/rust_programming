use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let mut t_num: i32 = t.trim().parse().unwrap();
    while t_num > 0 {
        test();
        t_num -= 1;
    }
}
fn test() {
    let mut size_arr = String::new();
    io::stdin().read_line(&mut size_arr).unwrap();
    let mut nk_string = String::new();
    io::stdin().read_line(&mut nk_string).unwrap();
    let data: Vec<i32> = nk_string
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // let data: Vec<i32> = vec![7, 4];
    let sum: i32 = data.iter().sum();
    let n = data.len();
    let mut should_break = false;
    for start in 0..n {
        if should_break {
            break;
        }
        for end in start + 1..=n {
            let subarray = &data[start..end];
            let subarray_sum: i32 = subarray.iter().sum();
            if check_pairity(sum - subarray_sum, subarray_sum) {
                // println!("sum = {} || sub_sum = {}", sum, subarray_sum);
                should_break = true;
                break;
            }
            // println!("{subarray:?}");
        }
    }
    if should_break {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn check_pairity(x: i32, y: i32) -> bool {
    if x == y {
        // println!("1");
        return true;
    };
    if x % 2 == 0 && y % 2 == 0 {
        // println!("2");
        return true;
    };
    if x % 2 == 1 && y % 2 == 1 {
        // println!("3");
        return true;
    };
    // println!("4");
    false
}

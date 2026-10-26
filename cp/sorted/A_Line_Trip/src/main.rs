use std::cmp;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn read_line_usr<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

fn main() {
    let t: usize = read_line_usr()[0];
    for _ in 0..t {
        test();
    }
}

fn test() {
    let nx: Vec<i32> = read_line_usr();
    let x = nx[1];

    let gas_stations: Vec<i32> = read_line_usr();

    let mut max_diff: i32 = gas_stations[0];

    for i in 1..gas_stations.len() {
        max_diff = cmp::max(gas_stations[i] - gas_stations[i - 1], max_diff);
    }

    let turnaround_diff = (x - gas_stations.last().unwrap()) * 2;
    let minimum_fuel = cmp::max(max_diff, turnaround_diff);

    println!("{}", minimum_fuel);
}

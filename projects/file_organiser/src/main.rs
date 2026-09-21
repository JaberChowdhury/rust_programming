mod utils;

use std::env;

use utils::populate::populate_messy_dir;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args[2]);

    let d = populate_messy_dir("projects/file_organiser", 12);
    match d {
        Ok(c) => dbg!(c),
        Err(r) => println!("{}", r),
    }
    Ok(())
}

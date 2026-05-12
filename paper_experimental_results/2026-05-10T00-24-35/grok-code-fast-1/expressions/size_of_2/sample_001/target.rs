use std::process;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let n = a.len() as i32;
    if n != 5 {
        process::exit(1);
    }
    process::exit(0);
}
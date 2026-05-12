use std::process::exit;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let n = a.len() as i32;
    if n != 5 {
        exit(1);
    }
    exit(0);
}
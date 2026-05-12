use std::process;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let n = a.len();
    if n != 5 {
        process::exit(1);
    }
    process::exit(0);
}
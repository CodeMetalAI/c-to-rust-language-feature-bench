use std::mem;
use std::process;

fn main() {
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let n = mem::size_of::<[i32; 5]>() / mem::size_of::<i32>();

    if n != 5 {
        process::exit(1);
    }
}
use std::mem::size_of;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let n = (size_of::<[i32; 5]>() / size_of::<i32>()) as i32;

    if n != 5 {
        std::process::exit(1);
    }
}
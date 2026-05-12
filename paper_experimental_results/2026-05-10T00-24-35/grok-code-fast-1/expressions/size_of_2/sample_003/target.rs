fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let n: i32 = a.len() as i32;
    if n != 5 {
        std::process::exit(1);
    }
    // implicit return 0 (exit code 0)
}
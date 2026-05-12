fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let n: usize = a.len();
    if n != 5 {
        std::process::exit(1);
    }
    // Exit code 0 by default
}
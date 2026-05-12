fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let n = std::mem::size_of_val(&a) / std::mem::size_of::<i32>();
    if n != 5 {
        std::process::exit(1);
    }
}
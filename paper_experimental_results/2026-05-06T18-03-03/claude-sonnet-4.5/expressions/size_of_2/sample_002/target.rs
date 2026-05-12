fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let n = a.len() as i32;

    if n != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}
fn main() {
    const TABSIZE: usize = 100;
    let a: [i32; TABSIZE] = [0; TABSIZE];

    let tabsize: i32 = 7;
    if a.len()!= 100 {
        std::process::exit(1);
    }
    if tabsize!= 7 {
        std::process::exit(2);
    }
}
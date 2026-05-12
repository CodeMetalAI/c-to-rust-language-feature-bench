fn main() -> i32 {
    let g = 9;
    let p = |x: i32| {
        if x == 7 && g == 9 {
            std::process::exit(0);
        } else {
            std::process::exit(2);
        }
    };
    p(7);
    3
}
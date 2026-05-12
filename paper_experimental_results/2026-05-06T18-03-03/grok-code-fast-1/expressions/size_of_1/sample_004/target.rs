use std::process;

fn main() {
    let mut dp = Box::new(0.0);
    *dp = 3.14;
    if *dp != 3.14 {
        process::exit(2);
    }
    process::exit(0);
}
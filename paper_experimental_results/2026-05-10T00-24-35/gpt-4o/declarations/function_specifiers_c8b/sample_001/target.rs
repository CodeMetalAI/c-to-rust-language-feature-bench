use std::process::exit;

fn die_if(x: i32) -> ! {
    let g = 9;
    if x == 7 && g == 9 {
        exit(0);
    } else {
        exit(2);
    }
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    let g = 9;
    p(7);
}
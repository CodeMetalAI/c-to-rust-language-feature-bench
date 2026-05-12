use std::process::exit;

fn die_if(x: i32) -> ! {
    let g = 9;
    exit(if x == 7 && g == 9 { 0 } else { 2 });
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    p(7);
}
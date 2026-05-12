use std::process;

fn g: i32 = 0;

fn die_if(x: i32) {
    process::exit(if x == 7 && g == 9 { 0 } else { 2 });
}

fn main() {
    let p: fn(i32) = die_if;
    g = 9;
    p(7);
}
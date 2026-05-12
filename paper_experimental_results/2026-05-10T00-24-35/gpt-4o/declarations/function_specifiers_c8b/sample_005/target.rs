fn main() {
    let p: fn(i32) -> ! = die_if;
    g = 9;
    p(7);
}

static mut g: i32 = 0;

fn die_if(x: i32) -> ! {
    std::process::exit(if x == 7 && unsafe { g } == 9 { 0 } else { 2 });
}
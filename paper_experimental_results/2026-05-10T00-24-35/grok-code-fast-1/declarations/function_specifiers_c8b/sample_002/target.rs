fn die_if(x: i32) -> ! {
    std::process::exit(if x == 7 { 0 } else { 2 });
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    p(7);
}
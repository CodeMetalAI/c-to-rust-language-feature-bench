use std::cell::Cell;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn die_if(x: i32) -> ! {
    let g = G.get();
    if x == 7 && g == 9 {
        std::process::exit(0);
    } else {
        std::process::exit(2);
    }
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    G.set(9);
    p(7);
}
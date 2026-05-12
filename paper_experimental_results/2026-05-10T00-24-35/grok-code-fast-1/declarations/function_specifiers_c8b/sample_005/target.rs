use std::cell::RefCell;
use std::process;

thread_local!(static G: RefCell<i32> = RefCell::new(0));

fn die_if(x: i32) -> ! {
    G.with(|g| {
        if x == 7 && *g.borrow() == 9 {
            process::exit(0);
        } else {
            process::exit(2);
        }
    })
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    G.with(|g| *g.borrow_mut() = 9);
    p(7);
    return 3;
}
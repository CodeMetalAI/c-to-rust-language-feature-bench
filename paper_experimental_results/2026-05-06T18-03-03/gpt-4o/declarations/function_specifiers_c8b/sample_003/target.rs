use std::process::exit;

fn die_if(x: i32) -> ! {
    if x == 7 && G.with(|g| *g.borrow() == 9) {
        exit(0);
    } else {
        exit(2);
    }
}

thread_local! {
    static G: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    G.with(|g| *g.borrow_mut() = 9);
    p(7);
}
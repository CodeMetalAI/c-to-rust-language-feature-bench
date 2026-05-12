use std::cell::RefCell;

static G: RefCell<i32> = RefCell::new(0);

fn die_if(x: i32) -> ! {
    let g = *G.borrow();
    std::process::exit(if x == 7 && g == 9 { 0 } else { 2 });
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    *G.borrow_mut() = 9;
    p(7);
}
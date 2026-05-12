use std::cell::RefCell;
use std::process;

static HITS: RefCell<i32> = RefCell::new(0);

fn handler_a(x: i32) {
    *HITS.borrow_mut() += x + 1;
}

fn handler_b(x: i32) {
    *HITS.borrow_mut() += x + 2;
}

type fv = fn(i32);
type pfv = fn(i32);

fn signal(s: i32, h: fv) -> fv {
    if s & 1 != 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: pfv) -> pfv {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    *HITS.borrow()
}

fn call_signal_as_fv(s: i32, h: fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    *HITS.borrow()
}

fn call_signal_as_pfv(s: i32, h: pfv, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    *HITS.borrow()
}

fn main() {
    *HITS.borrow_mut() = 0;
    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        process::exit(1);
    }

    *HITS.borrow_mut() = 0;
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        process::exit(2);
    }

    *HITS.borrow_mut() = 0;
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        process::exit(3);
    }

    process::exit(0);
}
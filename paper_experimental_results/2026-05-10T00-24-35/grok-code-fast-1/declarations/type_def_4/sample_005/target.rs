use std::cell::RefCell;

thread_local! {
    static HITS: RefCell<i32> = RefCell::new(0);
}

fn handler_a(x: i32) {
    HITS.with(|h| *h.borrow_mut() += x + 1);
}

fn handler_b(x: i32) {
    HITS.with(|h| *h.borrow_mut() += x + 2);
}

type Fv = fn(i32);
type Pfv = fn(i32);

fn signal(s: i32, h: Fv) -> Fv {
    if s & 1 != 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: Pfv) -> Pfv {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    HITS.with(|h| *h.borrow())
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r: Fv = signal(s, h);
    r(arg);
    HITS.with(|h| *h.borrow())
}

fn call_signal_as_pfv(s: i32, h: Pfv, arg: i32) -> i32 {
    let r: Pfv = signal_alias(s, h);
    r(arg);
    HITS.with(|h| *h.borrow())
}

fn main() {
    HITS.with(|h| *h.borrow_mut() = 0);
    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        std::process::exit(1);
    }

    HITS.with(|h| *h.borrow_mut() = 0);
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        std::process::exit(2);
    }

    HITS.with(|h| *h.borrow_mut() = 0);
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}
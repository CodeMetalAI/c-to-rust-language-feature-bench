use std::cell::Cell;

thread_local! {
    static HITS: Cell<i32> = Cell::new(0);
}

fn handler_a(x: i32) {
    HITS.with(|h| h.set(h.get() + x + 1));
}

fn handler_b(x: i32) {
    HITS.with(|h| h.set(h.get() + x + 2));
}

fn signal(s: i32, h: fn(i32)) -> fn(i32) {
    if s & 1 != 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: fn(i32)) -> fn(i32) {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    HITS.with(|h| h.get())
}

fn call_signal_as_fv(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    HITS.with(|h| h.get())
}

fn call_signal_as_pfv(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    HITS.with(|h| h.get())
}

fn main() {
    HITS.with(|h| h.set(0));

    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        std::process::exit(1);
    }

    HITS.with(|h| h.set(0));
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        std::process::exit(2);
    }

    HITS.with(|h| h.set(0));
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        std::process::exit(3);
    }
}
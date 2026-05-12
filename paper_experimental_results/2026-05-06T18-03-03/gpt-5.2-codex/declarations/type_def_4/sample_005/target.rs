use std::cell::Cell;
use std::process;

type Fv = fn(i32);
type Pfv = fn(i32);

thread_local! {
    static HITS: Cell<i32> = Cell::new(0);
}

fn set_hits(v: i32) {
    HITS.with(|c| c.set(v));
}

fn add_hits(v: i32) {
    HITS.with(|c| c.set(c.get() + v));
}

fn get_hits() -> i32 {
    HITS.with(|c| c.get())
}

fn handler_a(x: i32) {
    add_hits(x + 1);
}

fn handler_b(x: i32) {
    add_hits(x + 2);
}

fn signal(s: i32, h: Fv) -> Fv {
    if (s & 1) != 0 {
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
    get_hits()
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    get_hits()
}

fn call_signal_as_pfv(s: i32, h: Pfv, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    get_hits()
}

fn main() {
    set_hits(0);

    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        process::exit(1);
    }

    set_hits(0);
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        process::exit(2);
    }

    set_hits(0);
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        process::exit(3);
    }

    process::exit(0);
}
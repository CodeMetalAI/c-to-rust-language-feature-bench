// type_def_4.rs
use std::cell::RefCell;

type Fv = dyn FnMut(i32) -> ();
type Pfv = dyn FnMut(i32) -> ();

static mut HITS: i32 = 0;

fn handler_a(x: i32) {
    unsafe { HITS += x + 1; }
}

fn handler_b(x: i32) {
    unsafe { HITS += x + 2; }
}

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

fn call_signal_as_plain(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_pfv(s: i32, h: Pfv, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    unsafe { HITS }
}

fn main() {
    unsafe { HITS = 0; }

    if call_signal_as_plain(1, handler_b, 3) != 4 {
        return 1;
    }

    unsafe { HITS = 0; }
    if call_signal_as_fv(2, handler_b, 4) != 6 {
        return 2;
    }

    unsafe { HITS = 0; }
    if call_signal_as_pfv(3, handler_b, 5) != 6 {
        return 3;
    }

    return 0;
}
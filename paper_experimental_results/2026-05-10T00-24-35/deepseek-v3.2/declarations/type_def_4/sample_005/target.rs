use std::sync::atomic::{AtomicI32, Ordering};

static HITS: AtomicI32 = AtomicI32::new(0);

fn handler_a(x: i32) {
    HITS.fetch_add(x + 1, Ordering::SeqCst);
}

fn handler_b(x: i32) {
    HITS.fetch_add(x + 2, Ordering::SeqCst);
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

fn call_signal_as_plain(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    HITS.load(Ordering::SeqCst)
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    HITS.load(Ordering::SeqCst)
}

fn call_signal_as_pfv(s: i32, h: Pfv, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    HITS.load(Ordering::SeqCst)
}

fn main() {
    HITS.store(0, Ordering::SeqCst);
    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        std::process::exit(1);
    }

    HITS.store(0, Ordering::SeqCst);
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        std::process::exit(2);
    }

    HITS.store(0, Ordering::SeqCst);
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}
type Fv = fn(i32);

static mut HITS: i32 = 0;

fn handler_a(x: i32) {
    unsafe {
        HITS += x + 1;
    }
}

fn handler_b(x: i32) {
    unsafe {
        HITS += x + 2;
    }
}

fn signal(s: i32, h: Option<Fv>) -> Fv {
    if s & 1 != 0 {
        handler_a
    } else {
        h.unwrap_or(handler_b)
    }
}

fn call_signal_as_plain(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, Some(h));
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r = signal(s, Some(h));
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_pfv(s: i32, h: Option<Fv>, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn main() -> i32 {
    unsafe {
        HITS = 0;
        if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
            return 1;
        }

        HITS = 0;
        if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
            return 2;
        }

        HITS = 0;
        if call_signal_as_pfv(3, Some(handler_b), 5) != (5 + 1) {
            return 3;
        }
    }

    0
}
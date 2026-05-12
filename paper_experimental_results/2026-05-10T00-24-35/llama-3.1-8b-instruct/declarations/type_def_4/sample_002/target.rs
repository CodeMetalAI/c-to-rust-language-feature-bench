// type_def_4.rs
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

type Fv = fn(i32);

fn signal(s: i32, h: Fv) -> Fv {
    if s & 1!= 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: Fv) -> Fv {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: fn(i32), arg: i32) -> i32 {
    let r: Fv = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32) -> i32 {
    let r: Fv = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_pfv(s: i32, h: Fv, arg: i32) -> i32 {
    let r: Fv = signal_alias(s, h);
    r(arg);
    unsafe { HITS }
}

fn main() {
    unsafe {
        HITS = 0;
    }

    if call_signal_as_plain(1, handler_b, 3)!= 4 {
        std::process::exit(1);
    }

    unsafe {
        HITS = 0;
    }
    if call_signal_as_fv(2, handler_b, 4)!= 6 {
        std::process::exit(2);
    }

    unsafe {
        HITS = 0;
    }
    if call_signal_as_pfv(3, handler_b, 5)!= 6 {
        std::process::exit(3);
    }

    std::process::exit(0);
}
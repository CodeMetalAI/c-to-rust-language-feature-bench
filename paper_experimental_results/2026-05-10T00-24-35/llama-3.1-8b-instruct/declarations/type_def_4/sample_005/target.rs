// type_def_4.rs
type Handler = fn(i32) -> ();
type Signal<'a> = &'a mut dyn FnMut(i32) -> ();

static mut HITS: i32 = 0;

fn handler_a(_x: i32) {
    unsafe { HITS += _x + 1 };
}

fn handler_b(_x: i32) {
    unsafe { HITS += _x + 2 };
}

fn signal(s: i32, h: impl Fn(i32) -> () + 'static) -> impl Fn(i32) -> () {
    if s & 1 != 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: impl Fn(i32) -> () + 'static) -> impl Fn(i32) -> () {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: impl Fn(i32) -> (), arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_fv(s: i32, h: impl Fn(i32) -> () + 'static, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_pfv(s: i32, h: impl Fn(i32) -> () + 'static, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    unsafe { HITS }
}

fn main() {
    unsafe { HITS = 0 };

    if call_signal_as_plain(1, handler_b, 3) != 4 {
        std::process::exit(1);
    }

    unsafe { HITS = 0 };
    if call_signal_as_fv(2, handler_b, 4) != 6 {
        std::process::exit(2);
    }

    unsafe { HITS = 0 };
    if call_signal_as_pfv(3, handler_b, 5) != 6 {
        std::process::exit(3);
    }

    std::process::exit(0);
}
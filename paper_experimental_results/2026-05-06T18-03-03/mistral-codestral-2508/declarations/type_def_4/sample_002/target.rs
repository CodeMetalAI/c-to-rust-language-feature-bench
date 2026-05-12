fn main() {
    let mut hits = 0;

    if call_signal_as_plain(1, handler_b, 3, &mut hits) != 3 + 1 {
        std::process::exit(1);
    }

    hits = 0;
    if call_signal_as_fv(2, handler_b, 4, &mut hits) != 4 + 2 {
        std::process::exit(2);
    }

    hits = 0;
    if call_signal_as_pfv(3, handler_b, 5, &mut hits) != 5 + 1 {
        std::process::exit(3);
    }
}

type Fv = fn(int: i32);

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

fn signal(s: i32, h: Fv) -> Fv {
    if s & 1 != 0 {
        handler_a
    } else {
        h
    }
}

fn signal_alias(s: i32, h: Fv) -> Fv {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: Fv, arg: i32, hits: &mut i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe {
        *hits = HITS;
    }
    *hits
}

fn call_signal_as_fv(s: i32, h: Fv, arg: i32, hits: &mut i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe {
        *hits = HITS;
    }
    *hits
}

fn call_signal_as_pfv(s: i32, h: Fv, arg: i32, hits: &mut i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    unsafe {
        *hits = HITS;
    }
    *hits
}
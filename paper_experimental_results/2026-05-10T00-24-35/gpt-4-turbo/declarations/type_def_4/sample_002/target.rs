fn main() {
    let mut hits = 0;

    let signal = |s: i32, h: fn(i32)| -> fn(i32) {
        if s & 1 != 0 {
            handler_a
        } else {
            h
        }
    };

    let call_signal_as_plain = |s: i32, h: fn(i32), arg: i32| -> i32 {
        let r = signal(s, h);
        r(arg);
        hits
    };

    hits = 0;
    if call_signal_as_plain(1, handler_b, 3) != (3 + 1) {
        return std::process::exit(1);
    }

    let call_signal_as_fv = |s: i32, h: fn(i32), arg: i32| -> i32 {
        let r = signal(s, h);
        r(arg);
        hits
    };

    hits = 0;
    if call_signal_as_fv(2, handler_b, 4) != (4 + 2) {
        return std::process::exit(2);
    }

    let signal_alias = |s: i32, h: fn(i32)| -> fn(i32) {
        signal(s, h)
    };

    let call_signal_as_pfv = |s: i32, h: fn(i32), arg: i32| -> i32 {
        let r = signal_alias(s, h);
        r(arg);
        hits
    };

    hits = 0;
    if call_signal_as_pfv(3, handler_b, 5) != (5 + 1) {
        return std::process::exit(3);
    }
}

fn handler_a(x: i32) {
    static mut HITS: i32 = 0;
    unsafe {
        HITS += x + 1;
    }
}

fn handler_b(x: i32) {
    static mut HITS: i32 = 0;
    unsafe {
        HITS += x + 2;
    }
}
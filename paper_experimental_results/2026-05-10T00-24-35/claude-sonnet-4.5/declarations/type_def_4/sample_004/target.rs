fn main() {
    let mut hits = 0;

    if call_signal_as_plain(1, handler_b, 3, &mut hits) != (3 + 1) {
        std::process::exit(1);
    }

    hits = 0;
    if call_signal_as_fv(2, handler_b, 4, &mut hits) != (4 + 2) {
        std::process::exit(2);
    }

    hits = 0;
    if call_signal_as_pfv(3, handler_b, 5, &mut hits) != (5 + 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}

fn handler_a(x: i32, hits: &mut i32) {
    *hits += x + 1;
}

fn handler_b(x: i32, hits: &mut i32) {
    *hits += x + 2;
}

fn signal(s: i32, h: fn(i32, &mut i32), hits: &mut i32) -> fn(i32, &mut i32) {
    if (s & 1) != 0 {
        return handler_a;
    }
    h
}

fn signal_alias(s: i32, h: fn(i32, &mut i32), hits: &mut i32) -> fn(i32, &mut i32) {
    signal(s, h, hits)
}

fn call_signal_as_plain(s: i32, h: fn(i32, &mut i32), arg: i32, hits: &mut i32) -> i32 {
    let r = signal(s, h, hits);
    r(arg, hits);
    *hits
}

fn call_signal_as_fv(s: i32, h: fn(i32, &mut i32), arg: i32, hits: &mut i32) -> i32 {
    let r = signal(s, h, hits);
    r(arg, hits);
    *hits
}

fn call_signal_as_pfv(s: i32, h: fn(i32, &mut i32), arg: i32, hits: &mut i32) -> i32 {
    let r = signal_alias(s, h, hits);
    r(arg, hits);
    *hits
}
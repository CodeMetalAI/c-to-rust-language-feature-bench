fn main() {
    let mut hits = 0;

    if call_signal_as_plain(&mut hits, 1, handler_b, 3) != (3 + 1) {
        std::process::exit(1);
    }

    hits = 0;
    if call_signal_as_fv(&mut hits, 2, handler_b, 4) != (4 + 2) {
        std::process::exit(2);
    }

    hits = 0;
    if call_signal_as_pfv(&mut hits, 3, handler_b, 5) != (5 + 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}

fn handler_a(hits: &mut i32, x: i32) {
    *hits += x + 1;
}

fn handler_b(hits: &mut i32, x: i32) {
    *hits += x + 2;
}

fn signal(hits: &mut i32, s: i32, h: fn(&mut i32, i32)) -> fn(&mut i32, i32) {
    if s & 1 != 0 {
        return handler_a;
    }
    h
}

fn signal_alias(hits: &mut i32, s: i32, h: fn(&mut i32, i32)) -> fn(&mut i32, i32) {
    signal(hits, s, h)
}

fn call_signal_as_plain(hits: &mut i32, s: i32, h: fn(&mut i32, i32), arg: i32) -> i32 {
    let r = signal(hits, s, h);
    r(hits, arg);
    *hits
}

fn call_signal_as_fv(hits: &mut i32, s: i32, h: fn(&mut i32, i32), arg: i32) -> i32 {
    let r = signal(hits, s, h);
    r(hits, arg);
    *hits
}

fn call_signal_as_pfv(hits: &mut i32, s: i32, h: fn(&mut i32, i32), arg: i32) -> i32 {
    let r = signal_alias(hits, s, h);
    r(hits, arg);
    *hits
}
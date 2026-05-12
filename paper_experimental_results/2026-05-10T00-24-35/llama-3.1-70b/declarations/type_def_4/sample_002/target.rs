fn main() {
    let mut hits = 0;

    fn handler_a(x: i32) {
        hits += x + 1;
    }

    fn handler_b(x: i32) {
        hits += x + 2;
    }

    let handler_a_ref = handler_a as fn(i32);
    let handler_b_ref = handler_b as fn(i32);

    fn signal(s: i32, h: fn(i32)) -> fn(i32) {
        if s & 1 == 1 {
            handler_a_ref
        } else {
            h
        }
    }

    let signal_alias = signal;

    fn call_signal_as_plain(s: i32, h: fn(i32), arg: i32) -> i32 {
        let r = signal(s, h);
        r(arg);
        hits
    }

    fn call_signal_as_fv(s: i32, h: fn(i32), arg: i32) -> i32 {
        let r = signal(s, h);
        r(arg);
        hits
    }

    fn call_signal_as_pfv(s: i32, h: fn(i32), arg: i32) -> i32 {
        let r = signal_alias(s, h);
        r(arg);
        hits
    }

    hits = 0;
    if call_signal_as_plain(1, handler_b_ref, 3)!= 4 {
        std::process::exit(1);
    }

    hits = 0;
    if call_signal_as_fv(2, handler_b_ref, 4)!= 6 {
        std::process::exit(2);
    }

    hits = 0;
    if call_signal_as_pfv(3, handler_b_ref, 5)!= 6 {
        std::process::exit(3);
    }

    std::process::exit(0);
}
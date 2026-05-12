fn main() {
    let mut hits = 0;

    if call_signal_as_plain(1, &handler_b, 3) != (3 + 1) {
        std::process::exit(1);
    }

    hits = 0;
    if call_signal_as_fv(2, &handler_b, 4) != (4 + 2) {
        std::process::exit(2);
    }

    hits = 0;
    if call_signal_as_pfv(3, &handler_b, 5) != (5 + 1) {
        std::process::exit(3);
    }

    0
}

type Fv = fn(i32) -> ();
type Pfv = fn(i32) -> ();

fn signal(s: i32, h: &Fv) -> Box<dyn Fn(i32) -> ()> {
    if s & 1 != 0 {
        Box::new(handler_a)
    } else {
        Box::new(h)
    }
}

fn signal_alias(s: i32, h: Pfv) -> Box<dyn Fn(i32) -> ()> {
    signal(s, &h as &Fv)
}

fn call_signal_as_plain(s: i32, h: &Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    hits += arg + if s & 1 != 0 { 1 } else { 0 };
    r(arg);
    hits
}

fn call_signal_as_fv(s: i32, h: &Fv, arg: i32) -> i32 {
    let r = signal(s, h);
    hits += arg + if s & 1 != 0 { 1 } else { 0 };
    r(arg);
    hits
}

fn call_signal_as_pfv(s: i32, h: Pfv, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    hits += arg + if s & 1 != 0 { 1 } else { 0 };
    r(arg);
    hits
}

fn handler_a(x: i32) {
    hits += x + 1;
}

fn handler_b(x: i32) {
    hits += x + 2;
}
type Fv = fn(int);

static mut HITS: i32 = 0;

static mut HANDLER_A: Option<Fv> = None;
static mut HANDLER_B: Option<Fv> = None;

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

fn signal(s: i32, h: Option<Fv>) -> Option<Fv> {
    if s & 1 != 0 {
        unsafe { HANDLER_A }
    } else {
        h
    }
}

fn signal_alias(s: i32, h: Option<Fv>) -> Option<Fv> {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: Option<Fv>, arg: i32) -> i32 {
    unsafe {
        let r = signal(s, h);
        if let Some(r) = r {
            r(arg);
        }
        HITS
    }
}

fn call_signal_as_fv(s: i32, h: Option<Fv>, arg: i32) -> i32 {
    unsafe {
        let r = signal(s, h);
        if let Some(r) = r {
            r(arg);
        }
        HITS
    }
}

fn call_signal_as_pfv(s: i32, h: Option<Fv>, arg: i32) -> i32 {
    unsafe {
        let r = signal_alias(s, h);
        if let Some(r) = r {
            r(arg);
        }
        HITS
    }
}

fn main() {
    unsafe {
        HITS = 0;
        HANDLER_A = Some(handler_a);
        HANDLER_B = Some(handler_b);
    }

    if call_signal_as_plain(1, unsafe { HANDLER_B }, 3) != 4 {
        std::process::exit(1);
    }

    unsafe {
        HITS = 0;
    }
    if call_signal_as_fv(2, unsafe { HANDLER_B }, 4) != 6 {
        std::process::exit(2);
    }

    unsafe {
        HITS = 0;
    }
    if call_signal_as_pfv(3, unsafe { HANDLER_B }, 5) != 6 {
        std::process::exit(3);
    }
}
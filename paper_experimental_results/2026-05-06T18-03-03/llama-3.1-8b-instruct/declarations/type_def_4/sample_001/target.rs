// type_def_4.rs
struct HandlerA;
struct HandlerB;

impl HandlerA {
    fn call(&self, x: i32) {
        static mut HITS: i32 = 0;
        unsafe {
            HITS += x + 1;
        }
    }
}

impl HandlerB {
    fn call(&self, x: i32) {
        static mut HITS: i32 = 0;
        unsafe {
            HITS += x + 2;
        }
    }
}

fn signal(s: i32, h: &dyn Fn(i32) + Send) -> Box<dyn Fn(i32) + Send> {
    if s & 1 != 0 {
        Box::new(HandlerA)
    } else {
        Box::new(h)
    }
}

fn signal_alias(s: i32, h: &dyn Fn(i32) + Send) -> Box<dyn Fn(i32) + Send> {
    signal(s, h)
}

fn call_signal_as_plain(s: i32, h: &dyn Fn(i32) + Send, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_fv(s: i32, h: &dyn Fn(i32) + Send, arg: i32) -> i32 {
    let r = signal(s, h);
    r(arg);
    unsafe { HITS }
}

fn call_signal_as_pfv(s: i32, h: &dyn Fn(i32) + Send, arg: i32) -> i32 {
    let r = signal_alias(s, h);
    r(arg);
    unsafe { HITS }
}

fn main() {
    static mut HITS: i32 = 0;

    if call_signal_as_plain(1, &HandlerB::call, 3) != 4 {
        panic!("Expected 4, got {}", call_signal_as_plain(1, &HandlerB::call, 3));
    }

    HITS = 0;
    if call_signal_as_fv(2, &HandlerB::call, 4) != 6 {
        panic!("Expected 6, got {}", call_signal_as_fv(2, &HandlerB::call, 4));
    }

    HITS = 0;
    if call_signal_as_pfv(3, &HandlerB::call, 5) != 6 {
        panic!("Expected 6, got {}", call_signal_as_pfv(3, &HandlerB::call, 5));
    }
}
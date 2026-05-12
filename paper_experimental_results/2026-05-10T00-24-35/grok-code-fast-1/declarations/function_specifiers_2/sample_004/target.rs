fn stop() -> ! {
    let mut x = 0;
    loop {
        x = x.wrapping_add(1);
    }
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    } else {
        // This case is unreachable in the given program, but to satisfy Rust's type system
        // for _Noreturn behavior, we make it diverge.
        loop {}
    }
}

fn main() {
    if true {
        f();
    }
    g(1);
}
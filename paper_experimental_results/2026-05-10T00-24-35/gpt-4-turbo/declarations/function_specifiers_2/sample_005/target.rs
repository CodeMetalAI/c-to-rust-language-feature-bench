fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop()
    } else {
        // Since stop() never returns, we shouldn't reach here, but g needs to not return.
        // The Rust compiler requires all branches to return the same type, so we mimic an infinite loop here too.
        loop {}
    }
}

fn main() {
    if true {
        f();
    }

    g(1);
}
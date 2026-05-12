fn stop() -> ! {
    let mut x: i32 = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    } else {
        loop {}
    }
}

fn main() {
    if true {
        f();
    }
    g(1);
    // The return 0 is unreachable in both C and Rust due to the infinite loop.
}
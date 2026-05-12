fn stop() -> ! {
    let mut x: i32 = 0;
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
    }
    loop {}
}

fn main() {
    if true {
        f();
    }

    g(1);
}
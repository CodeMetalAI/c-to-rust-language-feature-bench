fn stop() -> ! {
    let mut x: i32 = 0;
    loop {
        x = x.wrapping_add(1);
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop()
    } else {
        loop {}
    }
}

fn main() {
    if 1 == 1 {
        f();
    }

    g(1);
}
fn stop() -> ! {
    let mut x: i32 = 0;
    loop {
        x = x.wrapping_add(1);
        std::hint::spin_loop();
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop()
    } else {
        loop {
            std::hint::spin_loop();
        }
    }
}

fn main() {
    if true {
        f();
    }

    g(1);

    // unreachable
}
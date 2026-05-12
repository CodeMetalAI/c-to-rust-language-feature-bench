fn stop() -> ! {
    let mut x = 0i32;
    loop {
        x = x.wrapping_add(1);
        std::hint::black_box(x);
    }
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    } else {
        loop {
            std::hint::black_box(());
        }
    }
}

fn main() {
    if true {
        f();
    }
    g(1);
}
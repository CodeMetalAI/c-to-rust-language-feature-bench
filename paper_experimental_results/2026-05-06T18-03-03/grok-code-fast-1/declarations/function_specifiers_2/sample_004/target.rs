use std::hint::black_box;

fn stop() -> ! {
    let mut x = 0;
    loop {
        x = black_box(x + 1);
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
}
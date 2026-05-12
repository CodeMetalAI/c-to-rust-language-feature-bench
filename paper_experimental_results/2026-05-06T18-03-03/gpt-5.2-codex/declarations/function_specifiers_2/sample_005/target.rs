use std::hint::black_box;

fn stop() -> ! {
    let mut x: i32 = 0;
    loop {
        x = x.wrapping_add(1);
        black_box(x);
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
    if 1 != 0 {
        f();
    }

    g(1);
}
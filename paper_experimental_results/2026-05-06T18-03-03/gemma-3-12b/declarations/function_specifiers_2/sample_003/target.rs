use std::process;

#[inline(never)]
fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

#[inline(never)]
fn f() -> ! {
    stop();
}

#[inline(never)]
fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    }
}

fn main() -> i32 {
    if true {
        f();
    }

    g(1);

    0
}
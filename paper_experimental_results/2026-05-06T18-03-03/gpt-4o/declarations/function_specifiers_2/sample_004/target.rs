// function_specifiers_2.rs
fn stop() -> ! {
    let mut x = 0;
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
    }
    stop(); // To satisfy the return type of `!`, we need to call stop() unconditionally
}

fn main() {
    if true {
        f();
    }

    g(1);
}
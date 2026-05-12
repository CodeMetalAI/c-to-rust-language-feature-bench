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
    stop(); // This line is added to satisfy the return type `!`, ensuring the function never returns.
}

fn main() {
    if true {
        f();
    }

    g(1);
}
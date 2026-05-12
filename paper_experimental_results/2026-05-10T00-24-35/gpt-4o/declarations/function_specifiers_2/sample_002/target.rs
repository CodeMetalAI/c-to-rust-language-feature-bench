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
    stop(); // This line is unreachable, but necessary for return type `!`
}

fn main() {
    if true {
        f();
    }

    g(1);
}
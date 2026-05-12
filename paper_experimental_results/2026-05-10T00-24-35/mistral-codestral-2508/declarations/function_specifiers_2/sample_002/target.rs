fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop()
    } else {
        panic!("g() should not be called with i <= 0");
    }
}

fn main() {
    if true {
        f();
    }

    g(1);
}
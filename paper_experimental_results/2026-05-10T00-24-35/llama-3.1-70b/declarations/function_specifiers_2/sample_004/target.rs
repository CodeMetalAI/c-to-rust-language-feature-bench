fn stop() -> ! {
    let mut x: i32 = 0;
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
        panic!("g should not return")
    }
}

fn main() {
    if true {
        f();
    }

    g(1);

    // will not be reached due to f() call, but needed to compile
    std::process::exit(0);
}
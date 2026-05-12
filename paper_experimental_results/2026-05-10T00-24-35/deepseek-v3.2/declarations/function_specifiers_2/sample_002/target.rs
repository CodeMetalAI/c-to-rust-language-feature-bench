use std::process;

fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) {
    if i > 0 {
        stop();
    }
}

fn main() {
    if true {
        f();
    }
    
    g(1);
    
    // This point is unreachable, but Rust requires all code paths to return
    process::exit(0);
}
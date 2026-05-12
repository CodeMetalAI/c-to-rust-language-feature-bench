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
    // Non-diverging function must return normally when i <= 0
}

fn main() {
    if true {
        f();
    }
    
    // This line is unreachable since f() never returns
    // But Rust requires we handle the case where g() might return
    g(1);
    
    // This return is also unreachable, but Rust needs it for type checking
    process::exit(0);
}
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
        stop();
    }
    // the case where i <= 0 is not handled in C/C++ and would lead to undefined behavior,
    // but Rust needs all paths to be covered, or else it's a compile-time error.
    // Since the function is marked as diverging (-> !), we can safely call stop() here.
    stop()
}

fn main() {
    f();
    g(1);
}
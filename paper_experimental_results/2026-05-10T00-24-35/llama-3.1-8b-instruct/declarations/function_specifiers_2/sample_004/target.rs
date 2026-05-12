fn stop() {
    loop {
        let mut x = 0;
        x += 1;
    }
}

fn f() {
    stop();
}

fn g(i: i32) {
    if i > 0 {
        stop();
    }
}

fn main() {
    f();
    g(1);
}
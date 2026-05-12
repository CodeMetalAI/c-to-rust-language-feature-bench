fn _noreturn_stop() {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn _noreturn_f() {
    _noreturn_stop();
}

fn _noreturn_g(i: i32) {
    if i > 0 {
        _noreturn_stop();
    }
}

fn main() {
    _noreturn_f();

    _noreturn_g(1);

    std::process::exit(0);
}
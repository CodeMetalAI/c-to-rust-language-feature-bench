fn main() {
    let x = 10;
    let y = 20;

    let px = &x as *const i32;
    let py = &y as *const i32;

    let px_end = unsafe { px.offset(1) };
    let py_end = unsafe { py.offset(1) };

    if px != &x as *const i32 {
        std::process::exit(1);
    }

    if !(px_end > px) {
        std::process::exit(2);
    }

    if px_end != unsafe { (&x as *const i32).offset(1) } {
        std::process::exit(3);
    }

    if !(py_end > py) {
        std::process::exit(4);
    }
}
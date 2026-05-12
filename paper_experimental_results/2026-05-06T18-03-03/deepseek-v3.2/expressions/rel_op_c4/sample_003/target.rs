fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = px as *const i32 + 1;
    let py_end = py as *const i32 + 1;

    if px != &x {
        std::process::exit(1);
    }

    if !(px_end > px as *const i32) {
        std::process::exit(2);
    }

    if px_end != (&x as *const i32 + 1) {
        std::process::exit(3);
    }

    if !(py_end > py as *const i32) {
        std::process::exit(4);
    }

    std::process::exit(0);
}
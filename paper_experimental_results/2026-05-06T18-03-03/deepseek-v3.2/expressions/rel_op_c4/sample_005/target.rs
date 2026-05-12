fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = px as *const i32 + 1;
    let py_end = py as *const i32 + 1;

    if px != &x {
        return;
    }

    if !(px_end > px as *const i32) {
        return;
    }

    if px_end != (&x as *const i32 + 1) {
        return;
    }

    if !(py_end > py as *const i32) {
        return;
    }

    std::process::exit(0);
}
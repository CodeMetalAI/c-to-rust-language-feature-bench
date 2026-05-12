fn main() {
    let x_arr = [10];
    let y_arr = [20];

    let px = 0usize;
    let py = 0usize;

    let px_end = px + 1;
    let py_end = py + 1;

    if px != 0 {
        std::process::exit(1);
    }

    if !(px_end > px) {
        std::process::exit(2);
    }

    if px_end != 1 {
        std::process::exit(3);
    }

    if !(py_end > py) {
        std::process::exit(4);
    }

    std::process::exit(0);
}
use std::process::exit;

fn main() {
    let x = [10];
    let y = [20];

    let px: usize = 0;
    let py: usize = 0;

    let px_end = px + 1;
    let py_end = py + 1;

    if px != 0 {
        exit(1);
    }

    if !(px_end > px) {
        exit(2);
    }

    if px_end != 1 {
        exit(3);
    }

    if !(py_end > py) {
        exit(4);
    }
}
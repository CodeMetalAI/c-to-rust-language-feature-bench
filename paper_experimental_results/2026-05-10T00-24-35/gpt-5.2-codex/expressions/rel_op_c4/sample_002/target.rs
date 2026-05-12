use std::process::exit;

fn main() {
    let x: i32 = 10;
    let y: i32 = 20;

    let px = &x as *const i32 as usize;
    let py = &y as *const i32 as usize;

    let size = std::mem::size_of::<i32>();
    let px_end = px + size;
    let py_end = py + size;

    if px != (&x as *const i32 as usize) {
        exit(1);
    }

    if !(px_end > px) {
        exit(2);
    }

    if px_end != ((&x as *const i32 as usize) + size) {
        exit(3);
    }

    if !(py_end > py) {
        exit(4);
    }
}
use std::process::exit;

fn main() {
    let arr_x = [10i32, 0];
    let arr_y = [20i32, 0];

    let px = &arr_x[0] as *const i32 as usize;
    let py = &arr_y[0] as *const i32 as usize;

    let px_end = &arr_x[1] as *const i32 as usize;
    let py_end = &arr_y[1] as *const i32 as usize;

    if px != (&arr_x[0] as *const i32 as usize) {
        exit(1);
    }

    if !(px_end > px) {
        exit(2);
    }

    if px_end != (&arr_x[1] as *const i32 as usize) {
        exit(3);
    }

    if !(py_end > py) {
        exit(4);
    }
}
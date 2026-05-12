fn main() -> i32 {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = unsafe { ((px as *const _ as usize) + std::mem::size_of::<i32>()) as *const i32 };
    let py_end = unsafe { ((py as *const _ as usize) + std::mem::size_of::<i32>()) as *const i32 };

    if px != &x {
        return 1;
    }

    if !(px_end > px as *const i32) {
        return 2;
    }

    if px_end != (&x as *const i32).wrapping_offset(1) {
        return 3;
    }

    if !(py_end > py as *const i32) {
        return 4;
    }

    return 0;
}
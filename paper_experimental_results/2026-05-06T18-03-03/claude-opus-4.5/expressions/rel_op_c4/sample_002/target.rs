fn main() {
    let x = 10;
    let y = 20;

    let px = &x as *const i32;
    let py = &y as *const i32;

    let px_end = (px as usize) + std::mem::size_of::<i32>();
    let py_end = (py as usize) + std::mem::size_of::<i32>();

    let x_addr = &x as *const i32;
    if px != x_addr {
        std::process::exit(1);
    }

    if !(px_end > px as usize) {
        std::process::exit(2);
    }

    let x_plus_one = ((&x as *const i32) as usize) + std::mem::size_of::<i32>();
    if px_end != x_plus_one {
        std::process::exit(3);
    }

    if !(py_end > py as usize) {
        std::process::exit(4);
    }

    std::process::exit(0);
}
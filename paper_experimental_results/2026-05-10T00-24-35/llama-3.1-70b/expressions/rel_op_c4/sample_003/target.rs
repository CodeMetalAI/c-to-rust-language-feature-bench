fn main() {
    let x: i32 = 10;
    let y: i32 = 20;

    let px = &x as *const i32;
    let py = &y as *const i32;

    let px_end = unsafe { px.offset(1) };
    let py_end = unsafe { py.offset(1) };

    if px as *const _!= &x as *const _ {
        std::process::exit(1);
    }

    if!(px_end > px) {
        std::process::exit(2);
    }

    if px_end as *const _!= (&x as *const _).offset(1) as *const _ {
        std::process::exit(3);
    }

    if!(py_end > py) {
        std::process::exit(4);
    }

    std::process::exit(0);
}
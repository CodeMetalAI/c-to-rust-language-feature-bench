fn main() {
    let x = 10;
    let y = 20;

    // Using references instead of pointers
    let px = &x;
    let py = &y;

    // Get the reference to the next item in a hypothetical array
    let px_end = unsafe { px.add(1) };
    let py_end = unsafe { py.add(1) };

    if px as *const _ != &x as *const _ {
        std::process::exit(1);
    }

    if !(px_end > px) {
        std::process::exit(2);
    }

    if px_end as *const _ != (&x as *const _).wrapping_add(1) {
        std::process::exit(3);
    }

    if !(py_end > py) {
        std::process::exit(4);
    }
}
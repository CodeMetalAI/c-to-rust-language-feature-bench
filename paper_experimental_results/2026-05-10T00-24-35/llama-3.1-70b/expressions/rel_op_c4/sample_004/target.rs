fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = &x + 1;
    let py_end = &y + 1;

    if px as *const _!= &x as *const _ {
        return 1;
    }

    if!(*px_end as usize > *px as usize) {
        return 2;
    }

    if px_end as *const _!= (&x as *const _ + 1) {
        return 3;
    }

    if!(*py_end as usize > *py as usize) {
        return 4;
    }

    return 0;
}
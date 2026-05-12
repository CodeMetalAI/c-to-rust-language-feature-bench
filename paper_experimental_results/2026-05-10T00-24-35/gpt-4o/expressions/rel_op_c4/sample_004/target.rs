fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = unsafe { px.add(1) };
    let py_end = unsafe { py.add(1) };

    if std::ptr::eq(px, &x) == false {
        std::process::exit(1);
    }

    if !px_end > px {
        std::process::exit(2);
    }

    if std::ptr::eq(px_end, unsafe { &x.add(1) }) == false {
        std::process::exit(3);
    }

    if !py_end > py {
        std::process::exit(4);
    }
}
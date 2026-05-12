fn main() {
    let x = 10;
    let y = 20;
    let px = &x;
    let py = &y;

    let px_end = px.checked_add(1);
    let py_end = py.checked_add(1);

    if px != &x {
        return 1;
    }

    if px_end.unwrap() <= px {
        return 2;
    }

    if px_end.unwrap() != &x + 1 {
        return 3;
    }

    if py_end.unwrap() <= py {
        return 4;
    }

    return 0;
}
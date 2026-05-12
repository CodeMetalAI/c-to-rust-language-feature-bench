use std::cell::Cell;

fn main() {
    let i1 = Cell::new(1);
    let i2 = Cell::new(2);
    let i3 = Cell::new(3);
    let i4 = Cell::new(0);
    let i5 = Cell::new(0);

    let p_i1 = &i1;
    let p_i2 = &i2;
    let p_i4 = &i4;
    let p_i5 = &i5;

    if i1.get() != 1 {
        std::process::exit(1);
    }
    if i2.get() != 2 {
        std::process::exit(2);
    }
    if i3.get() != 3 {
        std::process::exit(3);
    }

    if i4.get() != 0 {
        std::process::exit(4);
    }
    if i5.get() != 0 {
        std::process::exit(5);
    }

    if !std::ptr::eq(p_i1, &i1) {
        std::process::exit(6);
    }
    if !std::ptr::eq(p_i2, &i2) {
        std::process::exit(7);
    }
    if !std::ptr::eq(p_i4, &i4) {
        std::process::exit(8);
    }
    if !std::ptr::eq(p_i5, &i5) {
        std::process::exit(9);
    }

    i1.set(10);
    i2.set(20);
    i4.set(30);
    i5.set(40);

    if p_i1.get() != 10 {
        std::process::exit(10);
    }
    if p_i2.get() != 20 {
        std::process::exit(11);
    }
    if p_i4.get() != 30 {
        std::process::exit(12);
    }
    if p_i5.get() != 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}
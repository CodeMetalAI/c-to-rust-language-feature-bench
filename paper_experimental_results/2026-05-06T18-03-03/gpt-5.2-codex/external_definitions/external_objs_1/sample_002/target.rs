use std::cell::Cell;
use std::ptr;

static I1: Cell<i32> = Cell::new(1);
static I2: Cell<i32> = Cell::new(2);
static I3: Cell<i32> = Cell::new(3);

static I4: Cell<i32> = Cell::new(0);
static I5: Cell<i32> = Cell::new(0);

static P_I1: &Cell<i32> = &I1;
static P_I2: &Cell<i32> = &I2;
static P_I4: &Cell<i32> = &I4;
static P_I5: &Cell<i32> = &I5;

fn main() {
    let code = if I1.get() != 1 {
        1
    } else if I2.get() != 2 {
        2
    } else if I3.get() != 3 {
        3
    } else if I4.get() != 0 {
        4
    } else if I5.get() != 0 {
        5
    } else if !ptr::eq(P_I1, &I1) {
        6
    } else if !ptr::eq(P_I2, &I2) {
        7
    } else if !ptr::eq(P_I4, &I4) {
        8
    } else if !ptr::eq(P_I5, &I5) {
        9
    } else {
        I1.set(10);
        I2.set(20);
        I4.set(30);
        I5.set(40);

        if P_I1.get() != 10 {
            10
        } else if P_I2.get() != 20 {
            11
        } else if P_I4.get() != 30 {
            12
        } else if P_I5.get() != 40 {
            13
        } else {
            0
        }
    };

    std::process::exit(code);
}
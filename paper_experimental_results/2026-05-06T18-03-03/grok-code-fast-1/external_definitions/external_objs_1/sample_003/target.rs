use std::cell::Cell;

static I1: Cell<i32> = Cell::new(1);
static I2: Cell<i32> = Cell::new(2);
static I3: i32 = 3;
static I4: Cell<i32> = Cell::new(0);
static I5: Cell<i32> = Cell::new(0);

static P_I1: &Cell<i32> = &I1;
static P_I2: &Cell<i32> = &I2;
static P_I4: &Cell<i32> = &I4;
static P_I5: &Cell<i32> = &I5;

fn main() {
    if I1.get() != 1 {
        std::process::exit(1);
    }
    if I2.get() != 2 {
        std::process::exit(2);
    }
    if I3 != 3 {
        std::process::exit(3);
    }
    if I4.get() != 0 {
        std::process::exit(4);
    }
    if I5.get() != 0 {
        std::process::exit(5);
    }
    if !std::ptr::eq(P_I1, &I1) {
        std::process::exit(6);
    }
    if !std::ptr::eq(P_I2, &I2) {
        std::process::exit(7);
    }
    if !std::ptr::eq(P_I4, &I4) {
        std::process::exit(8);
    }
    if !std::ptr::eq(P_I5, &I5) {
        std::process::exit(9);
    }
    I1.set(10);
    I2.set(20);
    I4.set(30);
    I5.set(40);
    if P_I1.get() != 10 {
        std::process::exit(10);
    }
    if P_I2.get() != 20 {
        std::process::exit(11);
    }
    if P_I4.get() != 30 {
        std::process::exit(12);
    }
    if P_I5.get() != 40 {
        std::process::exit(13);
    }
    std::process::exit(0);
}
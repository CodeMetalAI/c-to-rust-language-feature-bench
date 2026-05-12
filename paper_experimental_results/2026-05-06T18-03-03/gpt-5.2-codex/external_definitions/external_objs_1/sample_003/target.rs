use std::process::exit;
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

static I1: AtomicI32 = AtomicI32::new(1);
static I2: AtomicI32 = AtomicI32::new(2);
static I3: AtomicI32 = AtomicI32::new(3);

static I4: AtomicI32 = AtomicI32::new(0);
static I5: AtomicI32 = AtomicI32::new(0);

static P_I1: &AtomicI32 = &I1;
static P_I2: &AtomicI32 = &I2;
static P_I4: &AtomicI32 = &I4;
static P_I5: &AtomicI32 = &I5;

fn run() -> i32 {
    if I1.load(Ordering::SeqCst) != 1 {
        return 1;
    }
    if I2.load(Ordering::SeqCst) != 2 {
        return 2;
    }
    if I3.load(Ordering::SeqCst) != 3 {
        return 3;
    }

    if I4.load(Ordering::SeqCst) != 0 {
        return 4;
    }
    if I5.load(Ordering::SeqCst) != 0 {
        return 5;
    }

    if !ptr::eq(P_I1, &I1) {
        return 6;
    }
    if !ptr::eq(P_I2, &I2) {
        return 7;
    }
    if !ptr::eq(P_I4, &I4) {
        return 8;
    }
    if !ptr::eq(P_I5, &I5) {
        return 9;
    }

    I1.store(10, Ordering::SeqCst);
    I2.store(20, Ordering::SeqCst);
    I4.store(30, Ordering::SeqCst);
    I5.store(40, Ordering::SeqCst);

    if P_I1.load(Ordering::SeqCst) != 10 {
        return 10;
    }
    if P_I2.load(Ordering::SeqCst) != 20 {
        return 11;
    }
    if P_I4.load(Ordering::SeqCst) != 30 {
        return 12;
    }
    if P_I5.load(Ordering::SeqCst) != 40 {
        return 13;
    }

    0
}

fn main() {
    exit(run());
}
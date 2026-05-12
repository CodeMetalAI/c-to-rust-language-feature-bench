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

fn main() {
    if I1.load(Ordering::SeqCst) != 1 {
        exit(1);
    }
    if I2.load(Ordering::SeqCst) != 2 {
        exit(2);
    }
    if I3.load(Ordering::SeqCst) != 3 {
        exit(3);
    }

    if I4.load(Ordering::SeqCst) != 0 {
        exit(4);
    }
    if I5.load(Ordering::SeqCst) != 0 {
        exit(5);
    }

    if !ptr::eq(P_I1, &I1) {
        exit(6);
    }
    if !ptr::eq(P_I2, &I2) {
        exit(7);
    }
    if !ptr::eq(P_I4, &I4) {
        exit(8);
    }
    if !ptr::eq(P_I5, &I5) {
        exit(9);
    }

    I1.store(10, Ordering::SeqCst);
    I2.store(20, Ordering::SeqCst);
    I4.store(30, Ordering::SeqCst);
    I5.store(40, Ordering::SeqCst);

    if P_I1.load(Ordering::SeqCst) != 10 {
        exit(10);
    }
    if P_I2.load(Ordering::SeqCst) != 20 {
        exit(11);
    }
    if P_I4.load(Ordering::SeqCst) != 30 {
        exit(12);
    }
    if P_I5.load(Ordering::SeqCst) != 40 {
        exit(13);
    }

    exit(0);
}
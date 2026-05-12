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

fn main() -> i32 {
    if I1.load(Ordering::Relaxed) != 1 {
        return 1;
    }
    if I2.load(Ordering::Relaxed) != 2 {
        return 2;
    }
    if I3.load(Ordering::Relaxed) != 3 {
        return 3;
    }
    if I4.load(Ordering::Relaxed) != 0 {
        return 4;
    }
    if I5.load(Ordering::Relaxed) != 0 {
        return 5;
    }
    I1.store(10, Ordering::Relaxed);
    I2.store(20, Ordering::Relaxed);
    I4.store(30, Ordering::Relaxed);
    I5.store(40, Ordering::Relaxed);
    if P_I1.load(Ordering::Relaxed) != 10 {
        return 10;
    }
    if P_I2.load(Ordering::Relaxed) != 20 {
        return 11;
    }
    if P_I4.load(Ordering::Relaxed) != 30 {
        return 12;
    }
    if P_I5.load(Ordering::Relaxed) != 40 {
        return 13;
    }
    0
}
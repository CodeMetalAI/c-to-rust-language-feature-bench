use std::sync::atomic::{AtomicI32, Ordering};

static I1: AtomicI32 = AtomicI32::new(1);
static I2: AtomicI32 = AtomicI32::new(2);
static I3: AtomicI32 = AtomicI32::new(3);
static I4: AtomicI32 = AtomicI32::new(0);
static I5: AtomicI32 = AtomicI32::new(0);

fn main() {
    if I1.load(Ordering::SeqCst) != 1 {
        std::process::exit(1);
    }
    if I2.load(Ordering::SeqCst) != 2 {
        std::process::exit(2);
    }
    if I3.load(Ordering::SeqCst) != 3 {
        std::process::exit(3);
    }
    if I4.load(Ordering::SeqCst) != 0 {
        std::process::exit(4);
    }
    if I5.load(Ordering::SeqCst) != 0 {
        std::process::exit(5);
    }

    let p_i1 = &I1 as *const AtomicI32;
    let p_i2 = &I2 as *const AtomicI32;
    let p_i4 = &I4 as *const AtomicI32;
    let p_i5 = &I5 as *const AtomicI32;

    if p_i1 != &I1 as *const AtomicI32 {
        std::process::exit(6);
    }
    if p_i2 != &I2 as *const AtomicI32 {
        std::process::exit(7);
    }
    if p_i4 != &I4 as *const AtomicI32 {
        std::process::exit(8);
    }
    if p_i5 != &I5 as *const AtomicI32 {
        std::process::exit(9);
    }

    I1.store(10, Ordering::SeqCst);
    I2.store(20, Ordering::SeqCst);
    I4.store(30, Ordering::SeqCst);
    I5.store(40, Ordering::SeqCst);

    if I1.load(Ordering::SeqCst) != 10 {
        std::process::exit(10);
    }
    if I2.load(Ordering::SeqCst) != 20 {
        std::process::exit(11);
    }
    if I4.load(Ordering::SeqCst) != 30 {
        std::process::exit(12);
    }
    if I5.load(Ordering::SeqCst) != 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}
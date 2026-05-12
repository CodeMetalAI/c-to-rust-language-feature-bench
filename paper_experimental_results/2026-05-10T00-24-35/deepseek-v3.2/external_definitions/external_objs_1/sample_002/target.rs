use std::sync::atomic::{AtomicI32, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    static ref I1: AtomicI32 = AtomicI32::new(1);
    static ref I3: AtomicI32 = AtomicI32::new(3);
    static ref I4: AtomicI32 = AtomicI32::new(0);
}

static I2: AtomicI32 = AtomicI32::new(2);
static I5: AtomicI32 = AtomicI32::new(0);

fn main() {
    let p_i1_addr = &*I1 as *const AtomicI32;
    let p_i2_addr = &I2 as *const AtomicI32;
    let p_i4_addr = &*I4 as *const AtomicI32;
    let p_i5_addr = &I5 as *const AtomicI32;

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

    if &*I1 as *const AtomicI32 != p_i1_addr {
        std::process::exit(6);
    }
    if &I2 as *const AtomicI32 != p_i2_addr {
        std::process::exit(7);
    }
    if &*I4 as *const AtomicI32 != p_i4_addr {
        std::process::exit(8);
    }
    if &I5 as *const AtomicI32 != p_i5_addr {
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
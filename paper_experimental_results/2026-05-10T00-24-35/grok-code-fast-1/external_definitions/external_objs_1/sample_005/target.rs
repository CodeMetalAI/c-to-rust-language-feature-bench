use std::cell::RefCell;
use std::sync::LazyLock;

static I1: LazyLock<RefCell<i32>> = LazyLock::new(|| RefCell::new(1));
static I2: LazyLock<RefCell<i32>> = LazyLock::new(|| RefCell::new(2));
static I3: LazyLock<RefCell<i32>> = LazyLock::new(|| RefCell::new(3));
static I4: LazyLock<RefCell<i32>> = LazyLock::new(|| RefCell::new(0));
static I5: LazyLock<RefCell<i32>> = LazyLock::new(|| RefCell::new(0));

static P_I1: LazyLock<&'static RefCell<i32>> = LazyLock::new(|| &*I1);
static P_I2: LazyLock<&'static RefCell<i32>> = LazyLock::new(|| &*I2);
static P_I4: LazyLock<&'static RefCell<i32>> = LazyLock::new(|| &*I4);
static P_I5: LazyLock<&'static RefCell<i32>> = LazyLock::new(|| &*I5);

fn main() {
    use std::process;

    if *I1.borrow() != 1 {
        process::exit(1);
    }
    if *I2.borrow() != 2 {
        process::exit(2);
    }
    if *I3.borrow() != 3 {
        process::exit(3);
    }
    if *I4.borrow() != 0 {
        process::exit(4);
    }
    if *I5.borrow() != 0 {
        process::exit(5);
    }

    if P_I1.get().unwrap() as *const _ != &*I1 as *const _ {
        process::exit(6);
    }
    if P_I2.get().unwrap() as *const _ != &*I2 as *const _ {
        process::exit(7);
    }
    if P_I4.get().unwrap() as *const _ != &*I4 as *const _ {
        process::exit(8);
    }
    if P_I5.get().unwrap() as *const _ != &*I5 as *const _ {
        process::exit(9);
    }

    *I1.borrow_mut() = 10;
    *I2.borrow_mut() = 20;
    *I4.borrow_mut() = 30;
    *I5.borrow_mut() = 40;

    if *I1.borrow() != 10 {
        process::exit(10);
    }
    if *I2.borrow() != 20 {
        process::exit(11);
    }
    if *I4.borrow() != 30 {
        process::exit(12);
    }
    if *I5.borrow() != 40 {
        process::exit(13);
    }

    process::exit(0);
}
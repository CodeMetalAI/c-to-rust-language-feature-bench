use std::cell::RefCell;

static COUNTER: RefCell<i32> = RefCell::new(0);

fn f() -> i32 {
    let val;
    {
        let mut c = COUNTER.borrow_mut();
        *c += 7;
        val = *c;
    }
    val
}

fn g_ptr(func: fn() -> i32) -> i32 {
    let a = func();
    let b = func();
    a + b
}

fn g_fun(func: fn() -> i32) -> i32 {
    let a = func();
    let b = func();
    a + b
}

fn main() {
    *COUNTER.borrow_mut() = 0;

    let r1 = g_ptr(f);
    if *COUNTER.borrow() != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if *COUNTER.borrow() != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}
use std::ptr;

struct IntList {
    car: i32,
    cdr: *const IntList,
}

static mut ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: ptr::null(),
};

fn eval(x: &IntList) -> i32 {
    unsafe {
        if x.car != 0 {
            return 1;
        }
        if x.cdr != &ENDLESS_ZEROS as *const IntList {
            return 2;
        }
        if (*x.cdr).cdr != &ENDLESS_ZEROS as *const IntList {
            return 3;
        }
        0
    }
}

fn main() {
    unsafe {
        ENDLESS_ZEROS.cdr = &ENDLESS_ZEROS as *const IntList;
        std::process::exit(eval(&ENDLESS_ZEROS));
    }
}
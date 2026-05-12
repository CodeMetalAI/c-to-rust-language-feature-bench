/*
 * compound_literal_7 translated to Rust
 */

struct IntList {
    car: i32,
    cdr: *const IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: std::ptr::null(),
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr != &ENDLESS_ZEROS as *const IntList {
        return 2;
    }
    if unsafe { (*x.cdr).cdr } != &ENDLESS_ZEROS as *const IntList {
        return 3;
    }
    0
}

fn main() {
    // Initialize the `cdr` pointer of ENDLESS_ZEROS to point to itself
    unsafe {
        let endless_zeros_ptr = &ENDLESS_ZEROS as *const IntList as *mut IntList;
        (*endless_zeros_ptr).cdr = &ENDLESS_ZEROS;
    }
    std::process::exit(eval(&ENDLESS_ZEROS));
}
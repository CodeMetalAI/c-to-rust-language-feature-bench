struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !std::ptr::eq(x.cdr.as_deref(), Some(&endless_zeros())) {
        return 2;
    }
    if !std::ptr::eq(x.cdr.as_deref().and_then(|c| c.cdr.as_deref()), Some(&endless_zeros())) {
        return 3;
    }
    0
}

fn endless_zeros() -> &'static IntList {
    static mut ENDLESS_ZEROS: Option<IntList> = None;
    unsafe {
        ENDLESS_ZEROS.get_or_insert_with(|| IntList {
            car: 0,
            cdr: None,
        })
    }
}

fn main() {
    std::process::exit(eval(endless_zeros()));
}
struct IntList<'a> {
    car: i32,
    cdr: Option<&'a IntList<'a>>,
}

static ENDLESS_ZEROS: &IntList = &IntList {
    car: 0,
    cdr: None, // This will be updated after it is fully initialized
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        1
    } else if !std::ptr::eq(x.cdr.unwrap_or(&IntList { car: 0, cdr: None }), ENDLESS_ZEROS) {
        2
    } else if !std::ptr::eq(
        x.cdr.unwrap_or(&IntList { car: 0, cdr: None }).cdr.unwrap_or(&IntList { car: 0, cdr: None }),
        ENDLESS_ZEROS,
    ) {
        3
    } else {
        0
    }
}

fn main() {
    unsafe {
        let mut endless_zeros = IntList { car: 0, cdr: None };
        let endless_zeros_ref = &mut endless_zeros as *mut IntList;
        endless_zeros.cdr = Some(&*endless_zeros_ref);
    }

    std::process::exit(eval(&endless_zeros));
}
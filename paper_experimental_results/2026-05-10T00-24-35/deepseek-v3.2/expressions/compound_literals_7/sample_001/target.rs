struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: None,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if let Some(ref cdr) = x.cdr {
        if !std::ptr::eq(cdr.as_ref(), &ENDLESS_ZEROS) {
            return 2;
        }
        if let Some(ref cdr_cdr) = cdr.cdr {
            if !std::ptr::eq(cdr_cdr.as_ref(), &ENDLESS_ZEROS) {
                return 3;
            }
        } else {
            return 3;
        }
    } else {
        return 2;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
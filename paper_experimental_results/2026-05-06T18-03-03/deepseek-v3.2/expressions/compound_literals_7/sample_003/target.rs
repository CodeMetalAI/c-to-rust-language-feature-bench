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
    
    match &x.cdr {
        Some(cdr_box) => {
            if !std::ptr::eq(cdr_box as *const _, &ENDLESS_ZEROS as *const _) {
                return 2;
            }
            match &cdr_box.cdr {
                Some(next_cdr_box) => {
                    if !std::ptr::eq(next_cdr_box as *const _, &ENDLESS_ZEROS as *const _) {
                        return 3;
                    }
                }
                None => return 3,
            }
        }
        None => return 2,
    }
    
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Some(Box::new(IntList {
        car: 0,
        cdr: None,
    })),
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    match &x.cdr {
        Some(cdr) => {
            if cdr.car != 0 {
                return 2;
            }
            match &cdr.cdr {
                Some(cdr) => {
                    if cdr.car != 0 {
                        return 3;
                    }
                }
                None => (),
            }
        }
        None => (),
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
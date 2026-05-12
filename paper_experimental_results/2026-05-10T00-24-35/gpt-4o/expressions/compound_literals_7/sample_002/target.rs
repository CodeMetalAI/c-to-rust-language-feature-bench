struct IntList {
    car: i32,
    cdr: &'static IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 2;
    }
    if x.cdr.cdr as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 3;
    }
    0
}

fn main() -> i32 {
    eval(&ENDLESS_ZEROS)
}
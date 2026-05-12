struct IntList {
    car: i32,
    cdr: *const IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS as *const IntList,
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
    std::process::exit(eval(&ENDLESS_ZEROS));
}
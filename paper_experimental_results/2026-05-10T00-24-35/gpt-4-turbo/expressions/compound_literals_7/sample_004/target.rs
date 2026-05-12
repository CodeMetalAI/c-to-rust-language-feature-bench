struct IntList<'a> {
    car: i32,
    cdr: &'a IntList<'a>,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !std::ptr::eq(x.cdr, &ENDLESS_ZEROS) {
        return 2;
    }
    if !std::ptr::eq(x.cdr.cdr, &ENDLESS_ZEROS) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
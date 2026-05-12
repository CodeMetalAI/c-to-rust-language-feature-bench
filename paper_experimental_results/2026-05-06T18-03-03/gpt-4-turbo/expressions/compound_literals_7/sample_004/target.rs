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
        1
    } else if x.cdr as *const _ != &ENDLESS_ZEROS as *const _ {
        2
    } else if x.cdr.cdr as *const _ != &ENDLESS_ZEROS as *const _ {
        3
    } else {
        0
    }
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
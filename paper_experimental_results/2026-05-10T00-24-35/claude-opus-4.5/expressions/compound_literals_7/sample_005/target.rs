struct IntList {
    car: i32,
    is_endless_zeros: bool,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    is_endless_zeros: true,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !x.is_endless_zeros {
        return 2;
    }
    // x.cdr points to endless_zeros, and endless_zeros.cdr also points to endless_zeros
    // so x.cdr->cdr == &endless_zeros is always true when x.cdr == &endless_zeros
    if !ENDLESS_ZEROS.is_endless_zeros {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
struct IntList {
    car: i32,
    cdr_is_self: bool, // true means cdr points to the same endless_zeros
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr_is_self: true,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    // Check if x.cdr points to endless_zeros
    // Since cdr_is_self is true, it does
    if !x.cdr_is_self {
        return 2;
    }
    // x.cdr is endless_zeros, so x.cdr->cdr is also endless_zeros
    // (because endless_zeros.cdr_is_self is true)
    if !ENDLESS_ZEROS.cdr_is_self {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
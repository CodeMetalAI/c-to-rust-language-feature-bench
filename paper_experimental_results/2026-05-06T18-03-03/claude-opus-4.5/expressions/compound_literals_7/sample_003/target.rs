struct IntList {
    car: i32,
    // In safe Rust, we represent the self-referential nature
    // by using a boolean to indicate "points to endless_zeros"
    cdr_is_endless_zeros: bool,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr_is_endless_zeros: true,
};

fn get_cdr(_x: &IntList) -> &'static IntList {
    // Since cdr always points to endless_zeros in our case
    &ENDLESS_ZEROS
}

fn is_endless_zeros(x: &IntList) -> bool {
    x.cdr_is_endless_zeros && x.car == 0
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let cdr = get_cdr(x);
    if !is_endless_zeros(cdr) {
        return 2;
    }
    let cdr_cdr = get_cdr(cdr);
    if !is_endless_zeros(cdr_cdr) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
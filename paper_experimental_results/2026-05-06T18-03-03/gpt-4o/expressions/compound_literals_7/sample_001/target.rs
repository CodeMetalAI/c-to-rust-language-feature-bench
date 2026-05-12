struct IntList {
    car: i32,
    cdr: Option<&'static IntList>,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Some(&ENDLESS_ZEROS),
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr.unwrap() as *const _ != &ENDLESS_ZEROS as *const _ {
        return 2;
    }
    if x.cdr.unwrap().cdr.unwrap() as *const _ != &ENDLESS_ZEROS as *const _ {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
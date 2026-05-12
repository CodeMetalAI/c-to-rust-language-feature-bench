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
    if x.cdr.is_some() {
        return 2;
    }
    return 0;
}

fn main() {
    let result = eval(&ENDLESS_ZEROS);
    std::process::exit(result);
}
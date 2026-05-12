struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

static ENDLESS_ZEROS: Lazy<IntList> = Lazy::new(|| IntList {
    car: 0,
    cdr: Some(Box::new(ENDLESS_ZEROS.clone())),
});

fn eval(x: &IntList) -> i32 {
    if x.car!= 0 {
        return 1;
    }
    if x.cdr.as_ref().map(|cdr| cdr.car)!= Some(0) {
        return 2;
    }
    if x.cdr.as_ref().and_then(|cdr| cdr.cdr.as_ref()).map(|cdr| cdr.car)!= Some(0) {
        return 3;
    }
    return 0;
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
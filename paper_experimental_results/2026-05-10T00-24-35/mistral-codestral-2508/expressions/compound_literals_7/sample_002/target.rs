struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

impl IntList {
    fn new(car: i32, cdr: Option<Box<IntList>>) -> Self {
        IntList { car, cdr }
    }
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr.is_none() {
        return 2;
    }
    if x.cdr.as_ref().unwrap().cdr.is_none() {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = IntList::new(0, Some(Box::new(IntList::new(0, None))));
    std::process::exit(eval(&endless_zeros));
}
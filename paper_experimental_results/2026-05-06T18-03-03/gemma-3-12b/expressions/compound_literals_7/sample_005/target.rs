#[derive(Debug)]
struct IntList {
    car: i32,
    cdr: *const IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr != &ENDLESS_ZEROS {
        return 2;
    }
    if unsafe { (*x.cdr).cdr } != &ENDLESS_ZEROS {
        return 3;
    }
    return 0;
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
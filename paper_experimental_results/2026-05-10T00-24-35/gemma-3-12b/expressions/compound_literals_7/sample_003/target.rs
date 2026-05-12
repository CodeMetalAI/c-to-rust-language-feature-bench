#[derive(Debug)]
struct IntList {
    car: i32,
    cdr: *const IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS as *const IntList,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !x.cdr.is_null() && *x.cdr as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 2;
    }
    if !x.cdr.is_null() && (*x.cdr as *const IntList).cdr.is_null() == false && *((*x.cdr as *const IntList).cdr) as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 3;
    }
    return 0;
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
#[derive(PartialEq, Debug)]
struct IntList {
    car: i32,
    cdr: *mut IntList,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: &ENDLESS_ZEROS as *const IntList as *mut IntList,
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 2;
    }
    if unsafe { (*x.cdr).cdr } as *const IntList != &ENDLESS_ZEROS as *const IntList {
        return 3;
    }
    return 0;
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
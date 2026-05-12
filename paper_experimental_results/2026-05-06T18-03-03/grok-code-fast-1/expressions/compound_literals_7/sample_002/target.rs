use lazy_static::lazy_static;

#[derive(Clone, Copy)]
struct IntList {
    car: i32,
    cdr: &'static IntList,
}

lazy_static! {
    static ref ENDLESS_ZEROS: IntList = IntList {
        car: 0,
        cdr: &ENDLESS_ZEROS,
    };
}

fn eval(x: IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if (x.cdr as *const IntList) != (&*ENDLESS_ZEROS as *const IntList) {
        return 2;
    }
    if (x.cdr.cdr as *const IntList) != (&*ENDLESS_ZEROS as *const IntList) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(*ENDLESS_ZEROS));
}
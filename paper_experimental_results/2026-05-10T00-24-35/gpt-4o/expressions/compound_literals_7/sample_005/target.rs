struct IntList<'a> {
    car: i32,
    cdr: &'a IntList<'a>,
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr as *const _ != &endless_zeros as *const _ {
        return 2;
    }
    if x.cdr.cdr as *const _ != &endless_zeros as *const _ {
        return 3;
    }
    0
}

static endless_zeros: IntList = IntList {
    car: 0,
    cdr: &endless_zeros,
};

fn main() -> i32 {
    eval(&endless_zeros)
}
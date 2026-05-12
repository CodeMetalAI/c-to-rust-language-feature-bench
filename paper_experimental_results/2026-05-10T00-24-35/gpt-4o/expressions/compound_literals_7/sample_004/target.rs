struct IntList<'a> {
    car: i32,
    cdr: Option<&'a IntList<'a>>,
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if let Some(cdr) = x.cdr {
        if cdr as *const _ != x as *const _ {
            return 2;
        }
        if cdr.cdr != Some(&x) {
            return 3;
        }
    }
    0
}

fn main() -> i32 {
    let mut endless_zeros = IntList { car: 0, cdr: None };
    endless_zeros.cdr = Some(&endless_zeros);
    eval(&endless_zeros)
}
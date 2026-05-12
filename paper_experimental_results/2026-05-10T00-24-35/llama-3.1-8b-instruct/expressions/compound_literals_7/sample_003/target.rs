struct IntList {
    car: i32,
    cdr: Option<Box<Self>>,
}

impl IntList {
    fn new(car: i32) -> Self {
        Self { car, cdr: None }
    }
}

fn eval(x: &IntList) -> i32 {
    match (x.car!= 0, x.cdr.as_ref()) {
        (true, _) => 1,
        (false, Some(cdr)) if cdr == &IntList { car: 0, cdr: None } => 2,
        (false, Some(cdr)) if cdr.cdr == Some(&IntList { car: 0, cdr: None }) => 3,
        _ => 0,
    }
}

fn main() -> i32 {
    let endless_zeros = IntList { car: 0, cdr: Some(Box::new(endless_zeros)) };
    eval(&endless_zeros)
}
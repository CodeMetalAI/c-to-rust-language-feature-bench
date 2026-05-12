#[derive(Debug, PartialEq, Eq)]
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

fn eval(x: &IntList) -> i32 {
    if x.car!= 0 {
        return 1;
    }
    if x.cdr.as_ref().map_or(false, |cdr| cdr.car!= 0) {
        return 2;
    }
    if x.cdr.as_ref().and_then(|cdr| cdr.cdr.as_ref()).map_or(false, |cdr| cdr.car!= 0) {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = IntList {
        car: 0,
        cdr: Some(Box::new(IntList {
            car: 0,
            cdr: Some(Box::new(IntList {
                car: 0,
                cdr: None,
            })),
        })),
    };
    std::process::exit(eval(&endless_zeros));
}
#[derive(Debug, PartialEq, Eq)]
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

fn eval(x: &IntList) -> i32 {
    if x.car!= 0 {
        1
    } else if x.cdr.as_ref().map_or(false, |cdr| cdr.as_ref()!= &IntList { car: 0, cdr: None }) {
        2
    } else if x.cdr.as_ref().map_or(false, |cdr| cdr.cdr.as_ref().map_or(false, |cdr| cdr.as_ref()!= &IntList { car: 0, cdr: None })) {
        3
    } else {
        0
    }
}

fn main() {
    let endless_zeros = IntList { car: 0, cdr: Some(Box::new(IntList { car: 0, cdr: None })) };
    std::process::exit(eval(&endless_zeros));
}
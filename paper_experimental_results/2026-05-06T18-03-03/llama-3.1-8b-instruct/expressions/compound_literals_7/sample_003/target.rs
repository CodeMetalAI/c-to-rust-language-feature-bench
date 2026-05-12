// Define a struct to represent a linked list of integers
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

// Create a static instance of the linked list
static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Some(Box::new(ENDLESS_ZEROS)),
};

// Define a function to evaluate the linked list
fn eval(x: &IntList) -> i32 {
    match x.car {
        0 => match x.cdr {
            Some(ref y) => {
                if y.car != 0 {
                    1
                } else if y.cdr.as_ref().map_or(false, |z| z.car != 0) {
                    2
                } else if y.cdr.as_ref().and_then(|z| z.cdr.as_ref()).map_or(false, |w| w.car != 0) {
                    3
                } else {
                    0
                }
            }
            None => 0,
        },
        _ => 1,
    }
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
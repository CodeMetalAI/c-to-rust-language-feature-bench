// Define a struct to represent the linked list
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
    if x.car!= 0 {
        1
    } else if x.cdr.as_ref().unwrap().as_ref()!= &ENDLESS_ZEROS {
        2
    } else if x.cdr.as_ref().unwrap().cdr.as_ref().unwrap().as_ref()!= &ENDLESS_ZEROS {
        3
    } else {
        0
    }
}

// Define the main function
fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
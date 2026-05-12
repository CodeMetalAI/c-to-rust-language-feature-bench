// Define a struct to represent the linked list
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

// Create a static instance of the linked list with endless zeros
static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Some(Box::new(ENDLESS_ZEROS)),
};

// Define a function to evaluate the linked list
fn eval(x: &IntList) -> i32 {
    if x.car!= 0 {
        return 1;
    }
    if x.cdr.as_ref().unwrap().cdr.as_ref().unwrap()!= &ENDLESS_ZEROS {
        return 2;
    }
    if x.cdr.as_ref().unwrap().cdr.as_ref().unwrap().cdr.as_ref().unwrap()!= &ENDLESS_ZEROS {
        return 3;
    }
    0
}

fn main() {
    // Evaluate the endless zeros linked list and return the result
    std::process::exit(eval(&ENDLESS_ZEROS));
}
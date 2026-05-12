// Define a struct to represent the int_list
struct IntList {
    car: i32,
    cdr: Box<IntList>,
}

// Create a static instance of IntList with endless zeros
static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Box::new(ENDLESS_ZEROS),
};

// Define a function to evaluate the IntList
fn eval(x: &IntList) -> i32 {
    if x.car!= 0 {
        1
    } else if x.cdr.as_ref()!= &ENDLESS_ZEROS {
        2
    } else if x.cdr.cdr.as_ref()!= &ENDLESS_ZEROS {
        3
    } else {
        0
    }
}

// Define the main function
fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}
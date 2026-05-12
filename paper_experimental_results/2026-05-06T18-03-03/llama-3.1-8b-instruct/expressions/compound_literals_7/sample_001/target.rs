// Define a struct to represent a linked list of integers
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

// Define a static instance of the linked list
static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Some(Box::new(ENDLESS_ZEROS)),
};

// Define a function to evaluate the linked list
fn eval(x: &IntList) -> i32 {
    match x {
        IntList { car: 0, cdr: Some(_) } => 0,
        IntList { car: _, cdr: Some(_) } => 1,
        IntList { car: _, cdr: None } => 2,
        _ => 3,
    }
}

fn main() {
    // Call the eval function with the static instance
    println!("{}", eval(&ENDLESS_ZEROS));
}
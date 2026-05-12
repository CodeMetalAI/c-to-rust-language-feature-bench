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
    match x {
        IntList { car: 0, cdr: Some(_) } => 0,
        IntList { car: _, cdr: Some(ref y) } if y == &ENDLESS_ZEROS => 2,
        IntList { car: _, cdr: Some(ref y) } if y.cdr == Some(&ENDLESS_ZEROS) => 3,
        _ => 1,
    }
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
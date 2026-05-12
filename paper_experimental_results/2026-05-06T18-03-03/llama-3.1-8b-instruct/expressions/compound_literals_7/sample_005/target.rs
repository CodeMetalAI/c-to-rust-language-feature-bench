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
            Some(ref cdr) => match cdr.cdr {
                Some(ref cdr) => 3,
                None => 2,
            },
            None => 0,
        },
        _ => 1,
    }
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
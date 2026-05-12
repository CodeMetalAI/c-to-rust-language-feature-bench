// Define a struct to represent an integer list
struct IntList {
    car: i32,
    cdr: Option<Box<IntList>>,
}

// Create a reference to the endless zeros list
lazy_static! {
    static ref ENDLESS_ZEROS: IntList = IntList {
        car: 0,
        cdr: Some(Box::new(ENDLESS_ZEROS.clone())),
    };
}

// Define a function to evaluate the integer list
fn eval(x: &IntList) -> i32 {
    match x {
        IntList { car: 0, cdr: None } => 0,
        IntList { car: 0, cdr: Some(_)} => 2,
        IntList { car: 0, cdr: Some(cdr) } => {
            match cdr.cdr {
                None => 3,
                _ => 0,
            }
        },
        IntList { car: n, _ } => 1,
    }
}

fn main() {
    println!("{}", eval(&ENDLESS_ZEROS));
}
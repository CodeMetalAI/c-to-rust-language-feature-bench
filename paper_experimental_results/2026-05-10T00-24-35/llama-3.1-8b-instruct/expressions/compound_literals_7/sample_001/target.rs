// Define a recursive linked list structure
#[derive(Debug)]
enum IntList {
    Nil,
    Cons(i32, Box<IntList>),
}

impl IntList {
    fn new(car: i32, cdr: Box<IntList>) -> IntList {
        Self::Cons(car, cdr)
    }
}

fn eval(x: &IntList) -> i32 {
    match x {
        IntList::Nil => 0,
        IntList::Cons(car, cdr) => {
            match cdr {
                IntList::Nil => 1,
                IntList::Cons(_, cdrcdr) => {
                    match cdrcdr {
                        IntList::Nil => 2,
                        _ => 3,
                    }
                }
            }
        }
    }
}

fn main() {
    let endless_zeros = Box::new(IntList::Cons(0,
                                            Box::new(IntList::new(0,
                                                                 Box::new(endless_zeros.clone()))));
    println!("{}", eval(&endless_zeros));
    std::process::exit(eval(&endless_zeros) as i32);
}
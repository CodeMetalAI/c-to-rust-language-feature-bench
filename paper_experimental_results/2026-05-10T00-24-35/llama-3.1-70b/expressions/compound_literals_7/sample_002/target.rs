#[derive(Debug, PartialEq, Eq)]
enum IntList {
    Cons(i32, Box<IntList>),
    Nil,
}

fn eval(x: &IntList) -> i32 {
    match x {
        IntList::Cons(car, cdr) => {
            if *car!= 0 {
                return 1;
            }
            match **cdr {
                IntList::Cons(_, ref cdr cdr) => {
                    if **cdr!= IntList::Nil {
                        return 2;
                    }
                    match ***cdr {
                        IntList::Cons(_, ref cdr cdr cdr) => {
                            if **cdr!= IntList::Nil {
                                return 3;
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
            0
        }
        IntList::Nil => 0,
    }
}

fn main() {
    let endless_zeros = IntList::Cons(0, Box::new(IntList::Cons(0, Box::new(IntList::Nil))));
    std::process::exit(eval(&endless_zeros));
}
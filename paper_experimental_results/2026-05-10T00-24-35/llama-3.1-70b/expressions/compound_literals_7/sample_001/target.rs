#[derive(Debug, PartialEq, Eq)]
enum IntList {
    Cons(i32, Box<IntList>),
    EndlessZeros,
}

impl IntList {
    fn eval(&self) -> i32 {
        match self {
            IntList::Cons(car, cdr) => {
                if *car!= 0 {
                    1
                } else if **cdr!= IntList::EndlessZeros {
                    2
                } else if **cdr.cdr().unwrap()!= IntList::EndlessZeros {
                    3
                } else {
                    0
                }
            }
            IntList::EndlessZeros => unreachable!(),
        }
    }
}

fn main() {
    let endless_zeros = IntList::Cons(0, Box::new(IntList::EndlessZeros));
    std::process::exit(IntList::eval(&endless_zeros));
}
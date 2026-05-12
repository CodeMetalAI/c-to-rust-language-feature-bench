// Define a recursive linked list structure
struct Node {
    car: i32,
    cdr: Option<Box<Node>>,
}

static ENDLESS_ZEROS: Node = Node {
    car: 0,
    cdr: Some(Box::new(ENDLESS_ZEROS)),
};

fn eval(x: Node) -> i32 {
    match x.cdr {
        Some(ref y) if y.car != 0 => 1,
        Some(ref y) if y != &ENDLESS_ZEROS => 2,
        Some(ref y) => {
            match y.cdr {
                Some(ref z) if z != &ENDLESS_ZEROS => 3,
                _ => 0,
            }
        }
        None => 0,
    }
}

fn main() {
    println!("{}", eval(ENDLESS_ZEROS));
}
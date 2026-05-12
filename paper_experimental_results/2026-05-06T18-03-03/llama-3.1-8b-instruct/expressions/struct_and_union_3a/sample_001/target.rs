#[derive(Debug)]
enum Type {
    AllTypes,
    IntNode(i32),
    FloatNode(f64),
}

fn main() {
    let mut u = Type::FloatNode(3.14);

    if u != Type::FloatNode(3.14) {
        return 1;
    }
    if let Type::IntNode(_) = u {
        return 1;
    }
    if let Type::FloatNode(_) = u {
        return 2;
    }
    if u != Type::FloatNode(3.14) {
        return 3;
    }

    0
}
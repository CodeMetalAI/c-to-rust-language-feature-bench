use std::process::exit;

struct U {
    type_field: i32,
    intnode: i32,
    doublenode: f64,
}

impl U {
    fn new() -> Self {
        Self {
            type_field: 0,
            intnode: 0,
            doublenode: 0.0,
        }
    }
}

fn main() {
    let mut u = U::new();

    u.type_field = 1;
    u.doublenode = 3.14;

    if u.type_field != 1 {
        exit(1);
    }
    if u.type_field != 1 {
        exit(1);
    }
    if u.type_field != 1 {
        exit(2);
    }
    if u.doublenode != 3.14 {
        exit(3);
    }

    exit(0);
}
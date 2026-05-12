use std::process;

struct U {
    type_field: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        type_field: 0,
        doublenode: 0.0,
    };

    u.type_field = 1;
    u.doublenode = 3.14;

    if u.type_field != 1 {
        process::exit(1);
    }
    if u.type_field != 1 {
        process::exit(1);
    }
    if u.type_field != 1 {
        process::exit(2);
    }
    if u.doublenode != 3.14 {
        process::exit(3);
    }

    process::exit(0);
}
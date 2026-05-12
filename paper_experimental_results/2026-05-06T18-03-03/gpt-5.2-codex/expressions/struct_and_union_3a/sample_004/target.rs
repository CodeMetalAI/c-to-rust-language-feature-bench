use std::process::exit;

fn main() {
    let mut type_field: i32 = 0;
    let mut doublenode: f64 = 0.0;

    type_field = 1;
    doublenode = 3.14;

    if type_field != 1 {
        exit(1);
    }
    if type_field != 1 {
        exit(1);
    }
    if type_field != 1 {
        exit(2);
    }
    if doublenode != 3.14 {
        exit(3);
    }

    exit(0);
}
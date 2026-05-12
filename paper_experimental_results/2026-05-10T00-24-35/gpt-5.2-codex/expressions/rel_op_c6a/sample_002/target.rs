use std::process::exit;

fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 2;

    let expr1 = ((a < b) as i32) < c;
    if !expr1 {
        exit(1);
    }

    let expr2 = ((a < b) as i32) < c;
    if !expr2 {
        exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    let expr3 = ((a < b) as i32) < c;
    if !expr3 {
        exit(3);
    }

    exit(0);
}
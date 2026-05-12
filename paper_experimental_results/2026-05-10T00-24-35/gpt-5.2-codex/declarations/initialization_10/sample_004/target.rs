use std::process::exit;

struct DivT {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = DivT { quot: 2, rem: -1 };

    if answer.quot != 2 {
        exit(1);
    }
    if answer.rem != -1 {
        exit(2);
    }

    {
        let swapped = DivT { rem: -3, quot: 4 };
        if swapped.quot != 4 {
            exit(3);
        }
        if swapped.rem != -3 {
            exit(4);
        }
    }

    exit(0);
}
use std::process;

#[derive(Debug)]
struct DivT {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = DivT { quot: 2, rem: -1 };

    if answer.quot != 2 {
        process::exit(1);
    }
    if answer.rem != -1 {
        process::exit(2);
    }

    {
        let swapped = DivT { rem: -3, quot: 4 };
        if swapped.quot != 4 {
            process::exit(3);
        }
        if swapped.rem != -3 {
            process::exit(4);
        }
    }

    process::exit(0);
}
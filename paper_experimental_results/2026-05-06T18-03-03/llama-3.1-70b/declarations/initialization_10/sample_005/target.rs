#[derive(Debug, PartialEq)]
struct DivT {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = DivT { quot: 2, rem: -1 };

    if answer.quot!= 2 {
        std::process::exit(1);
    }
    if answer.rem!= -1 {
        std::process::exit(2);
    }

    {
        let swapped = DivT { rem: -3, quot: 4 };
        if swapped.quot!= 4 {
            std::process::exit(3);
        }
        if swapped.rem!= -3 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}
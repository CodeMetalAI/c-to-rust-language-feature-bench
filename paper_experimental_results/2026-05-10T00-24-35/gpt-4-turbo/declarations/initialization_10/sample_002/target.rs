fn main() {
    struct DivT {
        quot: i32,
        rem: i32,
    }

    let answer = DivT { quot: 2, rem: -1 };

    if answer.quot != 2 {
        std::process::exit(1);
    }
    if answer.rem != -1 {
        std::process::exit(2);
    }

    {
        let swapped = DivT { quot: 4, rem: -3 };
        if swapped.quot != 4 {
            std::process::exit(3);
        }
        if swapped.rem != -3 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}
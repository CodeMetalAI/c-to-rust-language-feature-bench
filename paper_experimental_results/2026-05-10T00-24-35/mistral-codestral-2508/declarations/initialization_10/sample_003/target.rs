fn main() {
    let answer = (2, -1);

    if answer.0 != 2 {
        std::process::exit(1);
    }
    if answer.1 != -1 {
        std::process::exit(2);
    }

    {
        let swapped = (-3, 4);
        if swapped.1 != 4 {
            std::process::exit(3);
        }
        if swapped.0 != -3 {
            std::process::exit(4);
        }
    }
}
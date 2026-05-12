fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x = 7;
    let y = 3;

    if format!("Flag").split_whitespace().count() != 1 {
        std::process::exit(1);
    }
    if format!("X = {}\n", x).split_whitespace().count() != 2 {
        std::process::exit(2);
    }

    if !streq("The first, second, and third items.", "The first, second, and third items.") {
        std::process::exit(3);
    }

    if !(x > y) && format!("x is {} but y is {}", x, y).split_whitespace().count() != 3 {
        std::process::exit(4);
    }
    if !(x < y) && format!("x is {} but y is {}", x, y).split_whitespace().count() != 3 {
        std::process::exit(5);
    }
}
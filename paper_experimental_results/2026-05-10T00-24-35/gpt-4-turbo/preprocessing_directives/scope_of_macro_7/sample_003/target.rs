fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn debug(args: usize) -> usize {
    args
}

fn showlist(args: &str) -> &str {
    args
}

fn report(test: bool, args: usize) -> usize {
    if test {
        1
    } else {
        args
    }
}

fn main() {
    let x = 7;
    let y = 3;

    if debug(1) != 1 {
        std::process::exit(1);
    }
    if debug(2) != 2 {
        std::process::exit(2);
    }

    if !streq(showlist("The first, second, and third items."),
              "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report(x > y, 2) != 1 {
        std::process::exit(4);
    }
    if report(x < y, 3) != 3 {
        std::process::exit(5);
    }
}
fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        return std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        return std::process::exit(2);
    }

    if showlist!("The first", "second", "and third items.") != "The first, second, and third items." {
        return std::process::exit(3);
    }

    if report(x > y, "x is {} but y is {}", x, y) != 1 {
        return std::process::exit(4);
    }
    if report(x < y, "x is {} but y is {}", x, y) != 3 {
        return std::process::exit(5);
    }
}

fn debug(args: &str, _vars: impl std::fmt::Display) -> usize {
    2
}

fn debug(args: &str) -> usize {
    1
}

fn showlist!(...args: &str) -> String {
    args.join(", ")
}

fn report(condition: bool, _format: &str, _vars: impl std::fmt::Display) -> usize {
    if condition { 1 } else { 3 }
}
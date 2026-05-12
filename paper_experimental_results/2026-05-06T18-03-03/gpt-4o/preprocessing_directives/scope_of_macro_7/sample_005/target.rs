fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn debug(args: &[&str]) -> usize {
    args.len()
}

fn showlist(args: &[&str]) -> String {
    args.join(", ")
}

fn report(test: bool, args: &[&str]) -> usize {
    if test {
        1
    } else {
        args.len()
    }
}

fn main() {
    let x = 7;
    let y = 3;

    if debug(&["Flag"]) != 1 {
        std::process::exit(1);
    }
    if debug(&["X = %d\n", &x.to_string()]) != 2 {
        std::process::exit(2);
    }

    if !streq(
        &showlist(&["The first", "second", "and third items."]),
        "The first, second, and third items.",
    ) {
        std::process::exit(3);
    }

    if report(x > y, &["x is %d but y is %d", &x.to_string(), &y.to_string()]) != 1 {
        std::process::exit(4);
    }
    if report(x < y, &["x is %d but y is %d", &x.to_string(), &y.to_string()]) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}
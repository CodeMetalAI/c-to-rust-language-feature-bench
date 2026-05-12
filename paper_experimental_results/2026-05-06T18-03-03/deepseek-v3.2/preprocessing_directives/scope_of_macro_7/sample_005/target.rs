fn main() {
    let x = 7;
    let y = 3;

    // debug macro equivalent: count arguments
    if debug("Flag") != 1 {
        return std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        return std::process::exit(2);
    }

    // showlist macro equivalent: stringify arguments
    if !streq(
        showlist(The first, second, and third items.),
        "The first, second, and third items.",
    ) {
        return std::process::exit(3);
    }

    // report macro equivalent: conditionally return 1 or argument count
    if report(x > y, "x is %d but y is %d", x, y) != 1 {
        return std::process::exit(4);
    }
    if report(x < y, "x is %d but y is %d", x, y) != 3 {
        return std::process::exit(5);
    }

    std::process::exit(0);
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

// Helper to count arguments (simulate PP_NARG)
fn debug(args: &str) -> usize {
    1
}
fn debug(args: &str, _: i32) -> usize {
    2
}

// Helper to stringify arguments (simulate showlist)
fn showlist(args: &str) -> &str {
    args
}

// Helper for report macro
fn report(test: bool, args: &str) -> usize {
    if test {
        1
    } else {
        1
    }
}
fn report(test: bool, args: &str, _: i32, __: i32) -> usize {
    if test {
        1
    } else {
        3
    }
}
fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(
        showlist("The first", "second", "and third items."),
        "The first, second, and third items.",
    ) {
        std::process::exit(3);
    }

    if report(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }
}

fn debug<const N: usize>(args: [&str; N]) -> usize {
    N
}

fn showlist(args: &str, args2: &str, args3: &str) -> String {
    format!("{}, {}, {}", args, args2, args3)
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn report<const N: usize>(test: bool, args: [&str; N]) -> usize {
    if test {
        1
    } else {
        N
    }
}
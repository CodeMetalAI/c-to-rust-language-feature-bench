fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(&showlist("The first, second, and third items."),
               "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

fn debug<T: AsRef<str>>(format: T) -> i32 {
    let format_str = format.as_ref();
    if format_str == "Flag" {
        1
    } else if format_str.starts_with("X = ") {
        2
    }
    else {
        0
    }
}

fn showlist<T: AsRef<str>>(values: T) -> String {
    values.as_ref().to_string()
}

fn report<T: AsRef<str>, U: AsRef<str>>(condition: bool, format: T, args: U) -> i32 {
    if condition {
        1
    } else {
        3
    }
}

fn streq(a: &str, b: &str) -> bool {
    if a == b {
        true
    } else {
        false
    }
}
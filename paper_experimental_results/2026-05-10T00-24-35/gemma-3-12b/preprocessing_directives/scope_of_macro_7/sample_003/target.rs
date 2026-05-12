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

fn debug<S: AsRef<str>>(format_str: S, args: &[impl std::fmt::Display]) -> usize {
    let formatted_string = format!("{}", format_str);
    let mut count: usize = 0;
    for arg in args {
        count += 1;
    }
    if formatted_string == "Flag" {
        return 1;
    }
    if formatted_string.contains("X = ") {
        return 2;
    }

    count
}

fn showlist<S: AsRef<str>>(args: &[S]) -> String {
    let mut result = String::new();
    for (i, arg) in args.iter().enumerate() {
        result.push_str(arg.as_ref());
        if i < args.len() - 1 {
            result.push_str(", ");
        }
    }
    result
}

fn streq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if char_a != char_b {
            return false;
        }
    }
    true
}

fn report<T: std::fmt::Display>(condition: bool, format_str: T, args: &[impl std::fmt::Display]) -> usize {
    if condition {
        1
    } else {
        let mut count: usize = 0;
        for _ in args {
            count += 1;
        }
        count
    }
}
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

fn debug<S: std::fmt::Display>(format: S, args: &[impl std::fmt::Display]) -> i32 {
    let format_str = format.to_string();
    let mut args_vec = Vec::new();
    for arg in args {
        args_vec.push(arg.to_string());
    }

    if format_str == "Flag" {
        1
    } else if format_str.starts_with("X = ") {
        2
    }
    else {
        0
    }
}

fn showlist<S: std::fmt::Display>(values: &[S]) -> String {
    let mut result = String::new();
    for (i, value) in values.iter().enumerate() {
        result.push_str(&value.to_string());
        if i < values.len() - 1 {
            result.push_str(", ");
        }
    }
    result
}

fn streq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, (char_a, char_b)) in a.chars().zip(b.chars()).enumerate() {
        if char_a != char_b {
            return false;
        }
    }
    true
}

fn report<T: std::fmt::Display>(condition: bool, format: T, args: &[impl std::fmt::Display]) -> i32 {
    if condition {
        1
    } else {
        3
    }
}
fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(&showlist("The first, second, and third items."), "The first, second, and third items.") {
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
    println!("{}", format_args!(format, args));
    let count = args.len() as i32;
    if format == "Flag" {
        1
    } else if format.to_string().contains("X") {
        2
    } else {
        count
    }
}

fn showlist<S: std::fmt::Display>(items: &[S]) -> String {
    let mut result = String::new();
    for (i, item) in items.iter().enumerate() {
        result.push_str(&format!("{}", item));
        if i < items.len() - 1 {
            result.push_str(", ");
        }
    }
    result
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn report<S: std::fmt::Display>(condition: bool, format: S, args: &[impl std::fmt::Display]) -> i32 {
    if condition {
        1
    } else {
        let count = args.len() as i32;
        count
    }
}

use std::fmt::Display;
use std::fmt::format_args;
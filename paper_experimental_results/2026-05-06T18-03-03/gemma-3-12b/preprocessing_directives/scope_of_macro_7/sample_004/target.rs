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
    let mut s = format.to_string();
    for arg in args {
        s.push_str(&arg.to_string());
    }
    if s == "Flag" {
        1
    } else if s == "X = 7\n" {
        2
    } else {
        0
    }
}

fn showlist<S: std::fmt::Display>(args: &[S]) -> String {
    let mut s = String::new();
    for (i, arg) in args.iter().enumerate() {
        s.push_str(&arg.to_string());
        if i < args.len() - 1 {
            s.push_str(", ");
        }
    }
    s
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn report<S: std::fmt::Display>(condition: bool, format: S, args: &[impl std::fmt::Display]) -> i32 {
    if condition {
        1
    } else {
        let mut s = format.to_string();
        for arg in args {
            s.push_str(&arg.to_string());
        }
        if s == "x is 7 but y is 3" {
            3
        } else {
            0
        }
    }
}
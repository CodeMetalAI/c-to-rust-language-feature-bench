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
    match s.as_str() {
        "Flag" => 1,
        _ => {
            let mut count = 0;
            for _ in args {
                count += 1;
            }
            count
        }
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

fn report<F: Fn() -> bool, S: std::fmt::Display>(test: F, format: S, args: &[impl std::fmt::Display]) -> i32 {
    if test() {
        let mut s = format.to_string();
        for arg in args {
            s.push_str(&arg.to_string());
        }
        if s == "x is %d but y is %d" {
            1
        } else {
            1
        }
    } else {
        let mut count = 0;
        for _ in args {
            count += 1;
        }
        count
    }
}

fn streq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, (ca, cb)) in a.chars().zip(b.chars()).enumerate() {
        if ca != cb {
            return false;
        }
    }
    true
}
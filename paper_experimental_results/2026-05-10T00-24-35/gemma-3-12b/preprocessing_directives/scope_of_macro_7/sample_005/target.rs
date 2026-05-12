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

fn debug<S: AsRef<str>>(format_str: S) -> usize {
    let s = format_str.as_ref();
    let args: Vec<&str> = s.split(",").collect();
    args.len()
}

fn showlist<S: AsRef<str>>(format_str: S) -> String {
    format_str.as_ref().to_string()
}

fn report<T: Into<bool>, S: AsRef<str>>(test: T, format_str: S, args: ...) -> usize {
    let test_result = test.into();
    let s = format_str.as_ref();
    let args_vec: Vec<&str> = std::vec::try_concat(std::vec![s]).unwrap_or_default();
    args_vec.len()
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
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

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() -> i32 {
    let x = 7;
    let y = 3;

    if debug(&["Flag"]) != 1 {
        return 1;
    }
    if debug(&["X = %d\n", &x.to_string()]) != 2 {
        return 2;
    }

    if !streq(showlist(&["The first", "second", "and", "third", "items."]).as_str(),
               "The first, second, and third items.") {
        return 3;
    }

    if report(x > y, &["x is ", &x.to_string(), " but y is ", &y.to_string()]) != 1 {
        return 4;
    }
    if report(x < y, &["x is ", &x.to_string(), " but y is ", &y.to_string()]) != 3 {
        return 5;
    }

    0
}
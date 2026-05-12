fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() -> i32 {
    let x = 7;
    let y = 3;

    let debug = |args: &[&str]| args.len();

    if debug(&["Flag"]) != 1 {
        return 1;
    }
    if debug(&["X = %d\n", &x.to_string()]) != 2 {
        return 2;
    }

    let showlist = |args: &str| args;

    if !streq(
        showlist("The first, second, and third items."),
        "The first, second, and third items.",
    ) {
        return 3;
    }

    let report = |test: bool, args: &[&str]| if test { 1 } else { args.len() };

    if report(x > y, &["x is %d but y is %d", &x.to_string(), &y.to_string()]) != 1 {
        return 4;
    }
    if report(x < y, &["x is %d but y is %d", &x.to_string(), &y.to_string()]) != 3 {
        return 5;
    }

    0
}
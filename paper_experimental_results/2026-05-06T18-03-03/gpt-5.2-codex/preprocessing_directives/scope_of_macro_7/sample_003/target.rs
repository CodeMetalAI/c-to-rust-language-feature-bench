fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn report(test: bool, nargs: i32) -> i32 {
    if test { 1 } else { nargs }
}

fn main() {
    let x = 7;
    let y = 3;

    // debug("Flag") -> 1
    if 1 != 1 {
        std::process::exit(1);
    }
    // debug("X = %d\n", x) -> 2
    if 2 != 2 {
        std::process::exit(2);
    }

    let showlist_str = "The first, second, and third items.";
    if !streq(showlist_str, "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report(x > y, 3) != 1 {
        std::process::exit(4);
    }
    if report(x < y, 3) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}
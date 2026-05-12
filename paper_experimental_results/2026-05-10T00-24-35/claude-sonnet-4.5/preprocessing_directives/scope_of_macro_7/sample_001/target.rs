fn main() {
    let x = 7;
    let y = 3;

    if debug_1() != 1 {
        std::process::exit(1);
    }
    if debug_2() != 2 {
        std::process::exit(2);
    }

    if showlist() != "The first, second, and third items." {
        std::process::exit(3);
    }

    if report_true() != 1 {
        std::process::exit(4);
    }
    if report_false() != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

fn debug_1() -> i32 {
    1
}

fn debug_2() -> i32 {
    2
}

fn showlist() -> &'static str {
    "The first, second, and third items."
}

fn report_true() -> i32 {
    1
}

fn report_false() -> i32 {
    3
}
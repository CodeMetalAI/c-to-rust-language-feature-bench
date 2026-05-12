fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(7, 7) {
        std::process::exit(1);
    }
    if !expect_type(7, 7) {
        std::process::exit(2);
    }
    if !expect_type(7, 7) {
        std::process::exit(3);
    }
    if !expect_type(7, 7) {
        std::process::exit(4);
    }
    if !expect_type(7, 7) {
        std::process::exit(5);
    }
    if !expect_type(7, 7) {
        std::process::exit(6);
    }

    if !expect_type(8, 8) {
        std::process::exit(7);
    }
    if !expect_type(10, 10) {
        std::process::exit(8);
    }
    if !expect_type(12, 12) {
        std::process::exit(9);
    }

    if !expect_type(9, 9) {
        std::process::exit(10);
    }
    if !expect_type(11, 11) {
        std::process::exit(11);
    }
    if !expect_type(11, 11) {
        std::process::exit(12);
    }

    if !expect_type(8, 8) {
        std::process::exit(13);
    }
    if !expect_type(7, 7) {
        std::process::exit(14);
    }
}
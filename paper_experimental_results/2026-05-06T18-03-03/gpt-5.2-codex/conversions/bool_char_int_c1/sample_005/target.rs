use std::process;

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(7, 7) {
        process::exit(1);
    }
    if !expect_type(7, 7) {
        process::exit(2);
    }
    if !expect_type(7, 7) {
        process::exit(3);
    }
    if !expect_type(7, 7) {
        process::exit(4);
    }
    if !expect_type(7, 7) {
        process::exit(5);
    }
    if !expect_type(7, 7) {
        process::exit(6);
    }

    if !expect_type(8, 8) {
        process::exit(7);
    }
    if !expect_type(10, 10) {
        process::exit(8);
    }
    if !expect_type(12, 12) {
        process::exit(9);
    }

    if !expect_type(9, 9) {
        process::exit(10);
    }
    if !expect_type(11, 11) {
        process::exit(11);
    }
    if !expect_type(11, 11) {
        process::exit(12);
    }

    if !expect_type(8, 8) {
        process::exit(13);
    }
    if !expect_type(7, 7) {
        process::exit(14);
    }
}
use std::process::exit;

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(7, 7) {
        exit(1);
    }
    if !expect_type(7, 7) {
        exit(2);
    }
    if !expect_type(7, 7) {
        exit(3);
    }
    if !expect_type(7, 7) {
        exit(4);
    }
    if !expect_type(7, 7) {
        exit(5);
    }
    if !expect_type(7, 7) {
        exit(6);
    }

    if !expect_type(8, 8) {
        exit(7);
    }
    if !expect_type(10, 10) {
        exit(8);
    }
    if !expect_type(12, 12) {
        exit(9);
    }

    if !expect_type(9, 9) {
        exit(10);
    }
    if !expect_type(11, 11) {
        exit(11);
    }
    if !expect_type(11, 11) {
        exit(12);
    }

    if !expect_type(8, 8) {
        exit(13);
    }
    if !expect_type(7, 7) {
        exit(14);
    }

    exit(0);
}
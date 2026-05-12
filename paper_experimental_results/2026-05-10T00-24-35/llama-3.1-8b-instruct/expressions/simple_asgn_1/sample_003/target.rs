fn f() -> i32 {
    -1
}

fn main() {
    let c: char = f();

    let eq = c == -1;

    let char_is_signed = -(char::MAX as i32) < 0;

    if char_is_signed {
        assert_eq!(eq, true);
    } else {
        assert_eq!(eq, false);
    }

    // assert_eq! is used to show expected behavior, but the real program would return 0
    // This program should return 0 if char is signed, 2 if it's unsigned
    println!("char is signed: {}", if char_is_signed { "true" } else { "false" });
}
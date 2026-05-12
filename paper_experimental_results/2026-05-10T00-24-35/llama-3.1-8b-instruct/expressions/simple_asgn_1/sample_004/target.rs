fn f() -> i32 {
    -1
}

fn main() {
    let c: i32 = f();

    let eq: bool = c == -1;

    let char_is_signed: bool = (i32::MIN as i8) < 0;

    if char_is_signed {
        if !eq {
            return 1;
        }
    } else {
        if eq {
            return 2;
        }
    }

    0
}
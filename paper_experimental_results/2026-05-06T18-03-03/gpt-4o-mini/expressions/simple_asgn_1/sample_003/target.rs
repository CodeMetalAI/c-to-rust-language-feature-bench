fn f() -> i32 {
    -1
}

fn main() -> i32 {
    let c = f();

    let eq = (c == -1);

    let char_is_signed = (-1i8 < 0);

    if char_is_signed {
        if eq != true {
            return 1;
        }
    } else {
        if eq != false {
            return 2;
        }
    }

    0
}
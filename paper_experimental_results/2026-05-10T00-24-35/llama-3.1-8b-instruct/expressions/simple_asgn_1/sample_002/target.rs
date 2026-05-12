fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i32 = f();

    let eq = c == -1;

    let char_is_signed = (i32::MIN < 0);

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
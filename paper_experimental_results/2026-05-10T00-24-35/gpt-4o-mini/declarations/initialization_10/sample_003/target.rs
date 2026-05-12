struct DivT {
    quot: i32,
    rem: i32,
}

fn main() -> i32 {
    let answer = DivT { quot: 2, rem: -1 };

    if answer.quot != 2 {
        return 1;
    }
    if answer.rem != -1 {
        return 2;
    }

    {
        let swapped = DivT { rem: -3, quot: 4 };
        if swapped.quot != 4 {
            return 3;
        }
        if swapped.rem != -3 {
            return 4;
        }
    }

    return 0;
}
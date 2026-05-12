#[derive(Debug, PartialEq)]
struct div_t {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = div_t { quot: 2, rem: -1 };

    if answer.quot != 2 {
        return 1;
    }
    if answer.rem != -1 {
        return 2;
    }

    {
        let swapped = div_t { rem: -3, quot: 4 };
        if swapped.quot != 4 {
            return 3;
        }
        if swapped.rem != -3 {
            return 4;
        }
    }

    0
}
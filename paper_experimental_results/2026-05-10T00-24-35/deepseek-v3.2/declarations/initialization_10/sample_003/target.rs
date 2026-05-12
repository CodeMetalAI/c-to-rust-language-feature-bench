struct Div {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = Div { quot: 2, rem: -1 };

    if answer.quot != 2 {
        return;
    }
    if answer.rem != -1 {
        return;
    }

    {
        let swapped = Div { rem: -3, quot: 4 };
        if swapped.quot != 4 {
            return;
        }
        if swapped.rem != -3 {
            return;
        }
    }

    std::process::exit(0);
}
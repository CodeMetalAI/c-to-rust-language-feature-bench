#[derive(Debug, PartialEq)]
struct s {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&s> = None;
    let mut j = 0;

    loop {
        let q = p;
        p = Some(&s { i: j + 1 });
        j += 1;

        if j < 2 {
            continue;
        }

        break;
    }

    match (p, q) {
        (Some(p_val), Some(q_val)) => p_val == q_val && q_val.i == 1,
        (None, None) => true, // Handle the case where p and q are both None.
        _ => false,              // Handle other cases where one is None and the other isn't.
    }
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
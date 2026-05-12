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

        return (p == q) && (p.map_or(false, |x| x.i == 1));
    }
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
fn f() -> bool {
    let mut p: Option<&s> = None;
    let mut q: Option<&s> = None;
    let mut j = 0;

    loop {
        let current_p = p.take();
        p = Some(&s { i: j });
        j += 1;
        q = current_p;
        if j < 2 {
            continue;
        }
        break;
    }

    p.map(|p| p.i == 1).unwrap_or(false) && q.is_some()
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}

struct s {
    i: i32,
}
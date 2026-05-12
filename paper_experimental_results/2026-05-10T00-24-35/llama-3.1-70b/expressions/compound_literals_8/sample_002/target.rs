fn f() -> bool {
    let mut p: Option<&mut Struct> = None;
    let mut q: Option<&mut Struct> = None;
    let mut j = 0;

    loop {
        q = p;
        p = Some(&mut Struct { i: j });
        j += 1;

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    p.as_ref().map_or(false, |p| p.i == 1) && p == q
}

#[derive(Debug)]
struct Struct {
    i: i32,
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
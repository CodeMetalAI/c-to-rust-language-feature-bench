fn f() -> bool {
    let mut p: Option<&mut S> = None;
    let mut q: Option<&mut S> = None;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(&mut S { i: j });
        j += 1;
        if j < 2 {
            continue;
        }
        break;
    }

    if let (Some(p), Some(q)) = (p, q) {
        p as *const _ == q as *const _ && q.i == 1
    } else {
        false
    }
}

struct S {
    i: i32,
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}
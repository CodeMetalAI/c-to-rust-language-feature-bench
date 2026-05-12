struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<usize> = None;
    let mut q: Option<usize> = None;
    let mut j: i32 = 0;
    let mut temp = S { i: 0 };

    loop {
        q = p;
        temp.i = j;
        j += 1;
        p = Some(1);

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    (p == q) && (temp.i == 1)
}

fn main() {
    let exit_code = if f() { 0 } else { 1 };
    std::process::exit(exit_code);
}
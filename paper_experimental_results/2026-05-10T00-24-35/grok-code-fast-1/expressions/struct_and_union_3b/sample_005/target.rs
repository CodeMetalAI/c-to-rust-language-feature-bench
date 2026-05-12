#[derive(Clone, Copy)]
struct T1 {
    m: i32,
}

#[derive(Clone, Copy)]
struct T2 {
    m: i32,
}

#[derive(Clone, Copy)]
struct U12 {
    value: i32,
}

fn f(u: &mut U12) -> i32 {
    let m = u.value;
    if m < 0 {
        u.value = -u.value;
    }
    u.value
}

fn main() {
    let mut u = U12 { value: -7 };
    u.value = 7;

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u.value != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}
fn main() {
    let mut n = 0;

    if n!= 0 {
        std::process::abort();
    }

    n += 1;

    if n!= 0 {
        std::process::abort();
    }

    n += 1;

    if n!= 0 {
        std::process::abort();
    }

    let cn: i32 = 0;

    if cn!= 0 {
        std::process::abort();
    }

    n += 1;

    if n!= 0 {
        std::process::abort();
    }

    if n!= 0 {
        std::process::abort();
    }

    n += 1;

    if n!= 0 {
        std::process::abort();
    }

    let a = [0];
    let ca = [0];

    if!a.is_empty() {
        n += 1;
    }

    if n!= 0 {
        std::process::abort();
    }

    if!ca.is_empty() {
        n += 1;
    }

    if n!= 0 {
        std::process::abort();
    }

    if n!= 0 {
        std::process::abort();
    }

    n += 1;

    if n!= 0 {
        std::process::abort();
    }

    let s: i16 = 0;

    if s!= 0 {
        n += 1;
    }

    if n!= 0 {
        std::process::abort();
    }

    std::process::exit(0);
}
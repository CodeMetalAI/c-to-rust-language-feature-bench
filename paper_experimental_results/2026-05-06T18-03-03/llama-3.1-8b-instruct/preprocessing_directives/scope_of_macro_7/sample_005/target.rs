macro_rules! pp_cat {
    ($a:expr, $b:expr) => {
        concat!($a, $b)
    };
}

macro_rules! pp_arg_n {
    ($($rest:tt)*) => {{
        let mut n = 0;
        let mut rest = $rest;
        while rest.len() > 0 {
            rest.pop();
            n += 1;
        }
        n
    }};
}

macro_rules! pp_narg {
    () => {{
        let mut rest = std::env::args().skip(1).collect::<Vec<_>>();
        pp_arg_n!(rest)
    }};
    ($($rest:tt)*) => {{
        let mut rest = $rest;
        pp_arg_n!(rest)
    }};
}

macro_rules! debug {
    ($($rest:tt)*) => {{
        pp_narg!($($rest)*)
    }};
}

macro_rules! showlist {
    ($($rest:tt)*) => {{
        concat!($($rest)*)
    }};
}

fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(c1, c2)| c1 == c2)
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag")!= 1 {
        return 1;
    }
    if debug!("X = {}", x)!= 2 {
        return 2;
    }

    if!streq(showlist!("The first, second, and third items."), "The first, second, and third items.") {
        return 3;
    }

    if report(x > y, "x is {} but y is {}", x, y)!= 1 {
        return 4;
    }
    if report(x < y, "x is {} but y is {}", x, y)!= 3 {
        return 5;
    }

    return 0;
}

macro_rules! report {
    ($test:expr, $($rest:tt)*) => {{
        if $test {
            1
        } else {
            pp_narg!($($rest)*)
        }
    }};
}
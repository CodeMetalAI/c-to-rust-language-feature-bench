macro_rules! count_args {
    ($($expr:expr),* $(,)?) => {
        <[()]>::len(&[$(count_args!(@sub $expr)),*])
    };
    (@sub $expr:expr) => { () };
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => {
        count_args!($($args),*) as i32
    };
}

fn normalize_stringify(s: &str) -> String {
    let mut out = String::new();
    let mut prev_space = false;
    for ch in s.chars() {
        if ch == ' ' {
            prev_space = true;
            continue;
        }
        if prev_space {
            if matches!(ch, ',' | '.' | ';' | ':' | '!' | '?') {
                // skip space before punctuation
            } else {
                out.push(' ');
            }
            prev_space = false;
        }
        out.push(ch);
    }
    if prev_space {
        out.push(' ');
    }
    out
}

macro_rules! showlist {
    ($($t:tt)*) => {
        normalize_stringify(stringify!($($t)*))
    };
}

macro_rules! report {
    ($test:expr, $($args:expr),* $(,)?) => {
        if $test {
            1
        } else {
            count_args!($($args),*) as i32
        }
    };
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(&showlist!(The first, second, and third items.), "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}
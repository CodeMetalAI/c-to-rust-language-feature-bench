use std::process::exit;

macro_rules! count_args {
    () => { 0i32 };
    ($($args:tt),+ $(,)?) => {
        <[()]>::len(&[$(count_args!(@sub $args)),+]) as i32
    };
    (@sub $arg:tt) => { () };
}

macro_rules! debug {
    ($($args:tt)*) => {
        count_args!($($args),*)
    };
}

macro_rules! report {
    ($test:expr $(, $args:tt)*) => {{
        if $test {
            1i32
        } else {
            count_args!($($args),*)
        }
    }};
}

macro_rules! showlist {
    ($($args:tt)*) => {
        showlist_impl(stringify!($($args)*))
    };
}

fn is_punct(c: char) -> bool {
    matches!(c, ',' | '.' | ';' | ':' | '?' | '!' | ')' | ']' | '}')
}

fn showlist_impl(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == ' ' {
            if let Some(&next) = chars.peek() {
                if is_punct(next) {
                    continue;
                }
            }
        }
        out.push(c);
    }
    out
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        exit(2);
    }

    let s = showlist!(The first, second, and third items.);
    if !streq(&s, "The first, second, and third items.") {
        exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        exit(5);
    }
}
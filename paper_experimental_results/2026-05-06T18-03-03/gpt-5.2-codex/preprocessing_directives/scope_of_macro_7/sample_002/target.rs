macro_rules! count_args {
    () => { 0usize };
    ($($args:expr),+ $(,)?) => {
        <[()]>::len(&[$(count_args!(@sub $args)),+])
    };
    (@sub $x:expr) => { () };
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => {
        count_args!($($args),*) as i32
    };
}

macro_rules! report {
    ($test:expr, $($args:expr),* $(,)?) => {
        if $test { 1 } else { count_args!($($args),*) as i32 }
    };
}

macro_rules! showlist {
    ($($t:tt)*) => {
        showlist_impl(stringify!($($t)*))
    };
}

fn showlist_impl(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == ' ' {
            if let Some(&next) = chars.peek() {
                if next == ',' || next == '.' {
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
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    let s = showlist!(The first, second, and third items.);
    if !streq(&s, "The first, second, and third items.") {
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
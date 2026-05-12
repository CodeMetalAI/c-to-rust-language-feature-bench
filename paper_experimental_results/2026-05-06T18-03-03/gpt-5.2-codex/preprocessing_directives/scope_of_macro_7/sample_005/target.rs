macro_rules! count_args {
    () => {0usize};
    ($($args:expr),+ $(,)?) => {
        <[()]>::len(&[$(count_args!(@sub $args)),*])
    };
    (@sub $x:expr) => {()};
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => {
        count_args!($($args),*) as i32
    };
}

fn clean_punct(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    for i in 0..chars.len() {
        let c = chars[i];
        if c == ' ' {
            if i + 1 < chars.len() {
                let next = chars[i + 1];
                if next == ',' || next == '.' || next == ';' || next == ':' || next == '?' || next == '!' {
                    continue;
                }
            }
        }
        out.push(c);
    }
    out
}

macro_rules! showlist {
    ($($args:tt)*) => {
        clean_punct(stringify!($($args)*))
    };
}

macro_rules! report {
    ($test:expr $(, $args:expr)* $(,)?) => {
        if $test { 1i32 } else { count_args!($($args),*) as i32 }
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

    let list = showlist!(The first, second, and third items.);
    if !streq(&list, "The first, second, and third items.") {
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
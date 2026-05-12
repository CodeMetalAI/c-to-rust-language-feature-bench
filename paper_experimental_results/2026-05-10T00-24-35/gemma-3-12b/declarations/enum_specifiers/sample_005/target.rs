#[derive(PartialEq, Eq, Copy, Clone)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret,
    Winedark,
}

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
    }
}

fn main() {
    if let Some(val) = chartreuse_value() {
        if val != 0 {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }

    if let Some(val) = burgundy_value() {
        if val != 1 {
            std::process::exit(2);
        }
    } else {
        std::process::exit(2);
    }

    if let Some(val) = claret_value() {
        if val != 20 {
            std::process::exit(3);
        }
    } else {
        std::process::exit(3);
    }

    if let Some(val) = winedark_value() {
        if val != 21 {
            std::process::exit(4);
        }
    } else {
        std::process::exit(4);
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        std::process::exit(5);
    }
    if *cp == Hue::Burgundy {
        std::process::exit(6);
    }

    if check_vals(*cp) != 12 {
        std::process::exit(7);
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        std::process::exit(8);
    }

    std::process::exit(0);
}

fn chartreuse_value() -> Option<i32> {
    Some(0)
}

fn burgundy_value() -> Option<i32> {
    Some(1)
}

fn claret_value() -> Option<i32> {
    Some(20)
}

fn winedark_value() -> Option<i32> {
    Some(21)
}
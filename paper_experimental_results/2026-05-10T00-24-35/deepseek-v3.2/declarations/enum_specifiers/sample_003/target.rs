#[derive(Copy, Clone, PartialEq)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret = 20,
    Winedark,
}

impl Hue {
    fn as_int(&self) -> i32 {
        *self as i32
    }
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
    let mut col: Hue;
    let cp: &Hue;

    if Hue::Chartreuse.as_int() != 0 {
        std::process::exit(1);
    }
    if Hue::Burgundy.as_int() != 1 {
        std::process::exit(2);
    }
    if Hue::Claret.as_int() != 20 {
        std::process::exit(3);
    }
    if Hue::Winedark.as_int() != 21 {
        std::process::exit(4);
    }

    col = Hue::Claret;
    cp = &col;

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
    if check_vals(col) != 13 {
        std::process::exit(8);
    }

    std::process::exit(0);
}
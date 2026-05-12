#[derive(PartialEq, Debug)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret = 20,
    Winedark,
}

impl Hue {
    fn value(&self) -> i32 {
        match self {
            Hue::Chartreuse => 0,
            Hue::Burgundy => 1,
            Hue::Claret => 20,
            Hue::Winedark => 21,
        }
    }
}

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
        _ => 0,
    }
}

fn main() {
    use Hue::*;

    if Chartreuse.value() != 0 {
        std::process::exit(1);
    }
    if Burgundy.value() != 1 {
        std::process::exit(2);
    }
    if Claret.value() != 20 {
        std::process::exit(3);
    }
    if Winedark.value() != 21 {
        std::process::exit(4);
    }

    let mut col = Claret;
    let cp = &col;

    if *cp != Claret {
        std::process::exit(5);
    }
    if *cp == Burgundy {
        std::process::exit(6);
    }

    if check_vals(*cp) != 12 {
        std::process::exit(7);
    }

    col = Winedark;
    if check_vals(col) != 13 {
        std::process::exit(8);
    }
}
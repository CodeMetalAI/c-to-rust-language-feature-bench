#[derive(Debug, PartialEq, Eq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
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
}
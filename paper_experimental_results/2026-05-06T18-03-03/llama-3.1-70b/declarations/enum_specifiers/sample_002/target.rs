enum Hue {
    Chartreuse,
    Burgundy,
    Claret = 20,
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
    let col = Hue::Claret;
    let cp = &col;

    if Hue::Chartreuse as i32 != 0 {
        std::process::exit(1);
    }
    if Hue::Burgundy as i32 != 1 {
        std::process::exit(2);
    }
    if Hue::Claret as i32 != 20 {
        std::process::exit(3);
    }
    if Hue::Winedark as i32 != 21 {
        std::process::exit(4);
    }

    if *cp != Hue::Claret {
        std::process::exit(5);
    }
    if *cp == Hue::Burgundy {
        std::process::exit(6);
    }

    if check_vals(*cp) != 12 {
        std::process::exit(7);
    }

    let col = Hue::Winedark;
    if check_vals(col) != 13 {
        std::process::exit(8);
    }
}
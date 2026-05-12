#[derive(Clone, Copy, PartialEq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
}

fn check_vals(v: Hue) -> i32 {
    if v == Hue::Chartreuse {
        10
    } else if v == Hue::Burgundy {
        11
    } else if v == Hue::Claret {
        12
    } else if v == Hue::Winedark {
        13
    } else {
        0
    }
}

fn main() {
    let mut col: Hue;
    let cp: &Hue;

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
    if check_vals(*cp) != 13 {
        std::process::exit(8);
    }

    std::process::exit(0);
}
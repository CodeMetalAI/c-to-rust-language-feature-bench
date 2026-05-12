#[derive(Debug, Clone, Copy, PartialEq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    WineDark = 21,
}

fn check_vals(v: Hue) -> i32 {
    if v == Hue::Chartreuse {
        10
    } else if v == Hue::Burgundy {
        11
    } else if v == Hue::Claret {
        12
    } else if v == Hue::WineDark {
        13
    } else {
        0
    }
}

fn main() {
    use std::process;

    if (Hue::Chartreuse as i32) != 0 {
        process::exit(1);
    }
    if (Hue::Burgundy as i32) != 1 {
        process::exit(2);
    }
    if (Hue::Claret as i32) != 20 {
        process::exit(3);
    }
    if (Hue::WineDark as i32) != 21 {
        process::exit(4);
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        process::exit(5);
    }
    if *cp == Hue::Burgundy {
        process::exit(6);
    }

    if check_vals(*cp) != 12 {
        process::exit(7);
    }

    col = Hue::WineDark;
    if check_vals(*cp) != 13 {
        process::exit(8);
    }

    process::exit(0);
}
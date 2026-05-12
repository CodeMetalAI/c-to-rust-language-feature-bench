// enum_specifiers.rs

#[derive(Clone, Copy, PartialEq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    WineDark = 21,
}

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::WineDark => 13,
    }
}

fn main() -> i32 {
    if Hue::Chartreuse as i32 != 0 {
        return 1;
    }
    if Hue::Burgundy as i32 != 1 {
        return 2;
    }
    if Hue::Claret as i32 != 20 {
        return 3;
    }
    if Hue::WineDark as i32 != 21 {
        return 4;
    }

    let mut col = Hue::Claret;
    let cp = col;

    if cp != Hue::Claret {
        return 5;
    }
    if cp == Hue::Burgundy {
        return 6;
    }

    if check_vals(cp) != 12 {
        return 7;
    }

    col = Hue::WineDark;
    if check_vals(cp) != 13 {
        return 8;
    }

    0
}
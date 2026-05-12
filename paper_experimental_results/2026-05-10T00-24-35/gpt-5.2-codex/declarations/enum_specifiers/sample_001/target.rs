use std::process::exit;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
}

fn check_vals(v: Hue) -> i32 {
    if v == Hue::Chartreuse {
        return 10;
    }
    if v == Hue::Burgundy {
        return 11;
    }
    if v == Hue::Claret {
        return 12;
    }
    if v == Hue::Winedark {
        return 13;
    }
    0
}

fn main() {
    if Hue::Chartreuse as i32 != 0 {
        exit(1);
    }
    if Hue::Burgundy as i32 != 1 {
        exit(2);
    }
    if Hue::Claret as i32 != 20 {
        exit(3);
    }
    if Hue::Winedark as i32 != 21 {
        exit(4);
    }

    let mut col = Hue::Claret;
    let cp = &mut col;

    if *cp != Hue::Claret {
        exit(5);
    }
    if *cp == Hue::Burgundy {
        exit(6);
    }

    if check_vals(*cp) != 12 {
        exit(7);
    }

    *cp = Hue::Winedark;
    if check_vals(*cp) != 13 {
        exit(8);
    }

    exit(0);
}
#[repr(C)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
}

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy =>, 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
    }
}

fn main() {
    if Hue::Chartreuse as i32 != 0 {
        return;
    }
    if Hue::Burgundy as i32 != 1 {
        return;
    }
    if Hue::Claret as i32 != 20 {
        return;
    }
    if Hue::Winedark as i32 != 21 {
        return;
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        return;
    }
    if *cp == Hue::Burgundy {
        return;
    }

    if check_vals(*cp) != 12 {
        return;
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        return;
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret,
    Winedark,
}

static CHECK_VALS: fn(Hue) -> i32 = check_vals;

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
    }
}

fn main() {
    if (Hue::Chartreuse as i32) != 0 {
        return 1;
    }
    if (Hue::Burgundy as i32) != 1 {
        return 2;
    }
    if (Hue::Claret as i32) != 20 {
        return 3;
    }
    if (Hue::Winedark as i32) != 21 {
        return 4;
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        return 5;
    }
    if *cp == Hue::Burgundy {
        return 6;
    }

    if CHECK_VALS(*cp) != 12 {
        return 7;
    }

    col = Hue::Winedark;
    if CHECK_VALS(*cp) != 13 {
        return 8;
    }

    0
}
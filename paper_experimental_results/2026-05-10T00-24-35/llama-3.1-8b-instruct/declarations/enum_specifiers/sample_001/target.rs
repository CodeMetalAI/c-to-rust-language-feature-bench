// enum_specifiers.rs

enum Hue {
    Chartreuse,
    Burgundy,
    Claret = 20,
    Winedark,
}

fn check_vals(color: Hue) -> i32 {
    match color {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
        _ => 0,
    }
}

fn main() {
    let col = Hue::Claret;
    let cp = &col;

    if col as i32 != 0 {
        panic!("chartreuse is not 0");
    }
    if col as i32 != 1 {
        panic!("burgundy is not 1");
    }
    if col as i32 != 20 {
        panic!("claret is not 20");
    }
    if col as i32 != 21 {
        panic!("winedark is not 21");
    }

    if *cp != Hue::Claret {
        panic!("cp is not claret");
    }
    if *cp == Hue::Burgundy {
        panic!("cp is burgundy");
    }

    if check_vals(*cp) != 12 {
        panic!("check_vals(winedark) is not 12");
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        panic!("check_vals(winedark) is not 13");
    }
}
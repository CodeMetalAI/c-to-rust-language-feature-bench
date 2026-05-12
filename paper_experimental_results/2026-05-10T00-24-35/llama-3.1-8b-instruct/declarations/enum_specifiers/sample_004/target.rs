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
        _ => 0,
    }
}

fn main() {
    let col = Hue::Claret;
    let cp = &col;

    if col as i32!= 20 {
        panic!("Expected col to be 20");
    }
    if *cp!= Hue::Claret {
        panic!("Expected cp to be Claret");
    }
    if *cp == Hue::Burgundy {
        panic!("Expected cp to be Claret, not Burgundy");
    }
    if check_vals(*cp)!= 12 {
        panic!("Expected check_vals to return 12");
    }

    let col = Hue::Winedark;
    if check_vals(*cp)!= 13 {
        panic!("Expected check_vals to return 13");
    }
}
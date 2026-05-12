// enum_specifiers.rs

#[derive(Debug, PartialEq)]
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
    let mut col = Hue::Claret;
    let cp = &col;

    if col as i32 != 20 {
        panic!("Expected 20");
    }
    if *cp != Hue::Claret {
        panic!("Expected Hue::Claret");
    }
    if check_vals(*cp) != 12 {
        panic!("Expected 12");
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        panic!("Expected 13");
    }
}
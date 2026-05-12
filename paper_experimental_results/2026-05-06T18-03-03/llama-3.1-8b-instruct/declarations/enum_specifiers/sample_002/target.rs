// enum_specifiers.rs

#[derive(Debug, PartialEq)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret,
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

    if col as i32 != 2 {
        panic!("Expected chartreuse to be 2");
    }
    if *cp != Hue::Claret {
        panic!("Expected cp to be Claret");
    }
    if check_vals(*cp) != 12 {
        panic!("Expected check_vals to return 12");
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        panic!("Expected check_vals to return 13");
    }
}
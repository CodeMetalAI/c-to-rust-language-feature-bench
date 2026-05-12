#[derive(Clone, Copy, Debug, PartialEq)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
}

fn check_vals(v: Hue) -> i32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
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
    if (Hue::Winedark as i32) != 21 {
        process::exit(4);
    }

    let mut col = Hue::Claret;
    let cp: &Hue = &col;

    if *cp != Hue::Claret {
        process::exit(5);
    }
    if *cp == Hue::Burgundy {
        process::exit(6);
    }

    if check_vals(*cp) != 12 {
        process::exit(7);
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        process::exit(8);
    }

    process::exit(0);
}
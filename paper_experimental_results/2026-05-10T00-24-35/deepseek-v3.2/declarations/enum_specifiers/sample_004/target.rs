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
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
    }
}

fn main() {
    let chartreuse_val = Hue::Chartreuse as i32;
    if chartreuse_val != 0 {
        std::process::exit(1);
    }
    let burgundy_val = Hue::Burgundy as i32;
    if burgundy_val != 1 {
        std::process::exit(2);
    }
    let claret_val = Hue::Claret as i32;
    if claret_val != 20 {
        std::process::exit(3);
    }
    let winedark_val = Hue::Winedark as i32;
    if winedark_val != 21 {
        std::process::exit(4);
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        std::process::exit(5);
    }
    if *cp == Hue::Burgundy {
        std::process::exit(6);
    }

    if check_vals(*cp) != 12 {
        std::process::exit(7);
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        std::process::exit(8);
    }

    std::process::exit(0);
}
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark,
}

fn check_vals(v: Hue) -> u32 {
    match v {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret => 12,
        Hue::Winedark => 13,
    }
}

fn main() -> std::io::Result<()> {
    if Hue::Chartreuse as u32 != 0 {
        std::process::exit(1);
    }
    if Hue::Burgundy as u32 != 1 {
        std::process::exit(2);
    }
    if Hue::Claret as u32 != 20 {
        std::process::exit(3);
    }
    if Hue::Winedark as u32 != 21 {
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

    Ok(())
}
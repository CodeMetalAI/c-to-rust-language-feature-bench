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
    let col: Hue = Hue::Claret;
    let cp = &col;

    if col as i32!= 20 {
        println!("assertion failed");
        return;
    }

    if *cp!= Hue::Claret {
        println!("assertion failed");
        return;
    }

    if check_vals(*cp)!= 12 {
        println!("assertion failed");
        return;
    }

    col = Hue::Winedark;
    if check_vals(*cp)!= 13 {
        println!("assertion failed");
        return;
    }

    println!("all assertions passed");
}
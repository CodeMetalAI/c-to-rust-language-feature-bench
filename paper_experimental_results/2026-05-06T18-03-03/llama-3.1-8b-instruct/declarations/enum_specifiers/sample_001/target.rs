// enum_specifiers.rs

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
    }
}

fn main() {
    let mut col = Hue::Claret;
    let cp = &col;

    if (chartreuse as i32 != 0) {
        println!("Error: chartreuse is not 0");
        return;
    }
    if (burgundy as i32 != 1) {
        println!("Error: burgundy is not 1");
        return;
    }
    if (claret as i32 != 20) {
        println!("Error: claret is not 20");
        return;
    }
    if (winedark as i32 != 21) {
        println!("Error: winedark is not 21");
        return;
    }

    if *cp != Hue::Claret {
        println!("Error: *cp is not Claret");
        return;
    }
    if *cp == Hue::Burgundy {
        println!("Error: *cp is Burgundy");
        return;
    }

    if check_vals(*cp) != 12 {
        println!("Error: check_vals(*cp) is not 12");
        return;
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        println!("Error: check_vals(*cp) is not 13");
        return;
    }

    println!("All tests passed");
}
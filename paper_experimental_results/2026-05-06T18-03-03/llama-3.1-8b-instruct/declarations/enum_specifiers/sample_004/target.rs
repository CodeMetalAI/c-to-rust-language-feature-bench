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
        _ => 0,
    }
}

fn main() {
    let col: Hue = Hue::Claret;
    let cp: &Hue = &col;

    if col as i32!= 20 {
        println!("Error: chartreuse is not 0");
        std::process::exit(1);
    }
    if *cp as i32!= 20 {
        println!("Error: burgundy is not 1");
        std::process::exit(2);
    }
    if *cp as i32!= 20 {
        println!("Error: claret is not 20");
        std::process::exit(3);
    }
    if *cp as i32!= 21 {
        println!("Error: winedark is not 21");
        std::process::exit(4);
    }

    if *cp!= Hue::Claret {
        println!("Error: *cp is not claret");
        std::process::exit(5);
    }
    if *cp == Hue::Burgundy {
        println!("Error: *cp is burgundy");
        std::process::exit(6);
    }

    if check_vals(*cp)!= 12 {
        println!("Error: check_vals(*cp) is not 12");
        std::process::exit(7);
    }

    col = Hue::Winedark;
    if check_vals(*cp)!= 13 {
        println!("Error: check_vals(*cp) is not 13");
        std::process::exit(8);
    }

    println!("All tests passed");
    std::process::exit(0);
}
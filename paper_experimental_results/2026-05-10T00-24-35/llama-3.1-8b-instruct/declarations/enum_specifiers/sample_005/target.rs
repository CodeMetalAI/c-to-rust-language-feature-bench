// enum_specifiers.rs

#[derive(Debug, PartialEq)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret(u8),
    Winedark,
}

fn check_vals(hue: Hue) -> u8 {
    match hue {
        Hue::Chartreuse => 10,
        Hue::Burgundy => 11,
        Hue::Claret(_) => 12,
        Hue::Winedark => 13,
    }
}

fn main() {
    let col: Hue = Hue::Claret(20);

    if (col as u8) != 0 {
        println!("{}: failed", 1);
        std::process::exit(1);
    }
    if (col as u8) != 1 {
        println!("{}: failed", 2);
        std::process::exit(2);
    }
    if (col as u8) != 20 {
        println!("{}: failed", 3);
        std::process::exit(3);
    }
    if (col as u8) != 21 {
        println!("{}: failed", 4);
        std::process::exit(4);
    }

    let cp = &col;
    if *cp != Hue::Claret(20) {
        println!("{}: failed", 5);
        std::process::exit(5);
    }
    if *cp == Hue::Burgundy {
        println!("{}: failed", 6);
        std::process::exit(6);
    }

    if check_vals(*cp) != 12 {
        println!("{}: failed", 7);
        std::process::exit(7);
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        println!("{}: failed", 8);
        std::process::exit(8);
    }

    println!("Success");
    std::process::exit(0);
}
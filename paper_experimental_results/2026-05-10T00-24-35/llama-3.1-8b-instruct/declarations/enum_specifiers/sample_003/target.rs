// enum_specifiers.rs

#[derive(Debug, PartialEq, Eq)]
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
    let col = Hue::Claret;
    let cp = &col;

    if col as i32 != 20 {
        println!("Test failed: (int)claret != 20");
        std::process::exit(3);
    }

    if *cp != Hue::Claret {
        println!("Test failed: *cp != claret");
        std::process::exit(5);
    }

    if check_vals(*cp) != 12 {
        println!("Test failed: check_vals(*cp) != 12");
        std::process::exit(7);
    }

    let col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        println!("Test failed: check_vals(*cp) != 13");
        std::process::exit(8);
    }

    println!("All tests passed.");
    std::process::exit(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Hue {
    Chartreuse = 0,
    Burgundy = 1,
    Claret = 20,
    Winedark = 21,
}

fn check_vals(v: Hue) -> i32 {
    if v == Hue::Chartreuse {
        return 10;
    }
    if v == Hue::Burgundy {
        return 11;
    }
    if v == Hue::Claret {
        return 12;
    }
    if v == Hue::Winedark {
        return 13;
    }
    0
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    if Hue::Chartreuse as i32 != 0 {
        return 1;
    }
    if Hue::Burgundy as i32 != 1 {
        return 2;
    }
    if Hue::Claret as i32 != 20 {
        return 3;
    }
    if Hue::Winedark as i32 != 21 {
        return 4;
    }

    let mut col = Hue::Claret;
    let cp = &mut col;

    if *cp != Hue::Claret {
        return 5;
    }
    if *cp == Hue::Burgundy {
        return 6;
    }

    if check_vals(*cp) != 12 {
        return 7;
    }

    *cp = Hue::Winedark;
    if check_vals(*cp) != 13 {
        return 8;
    }

    0
}
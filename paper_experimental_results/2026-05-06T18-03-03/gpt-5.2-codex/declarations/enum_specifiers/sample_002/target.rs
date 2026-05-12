use std::cell::Cell;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
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
    if Hue::Chartreuse as i32 != 0 {
        std::process::exit(1);
    }
    if Hue::Burgundy as i32 != 1 {
        std::process::exit(2);
    }
    if Hue::Claret as i32 != 20 {
        std::process::exit(3);
    }
    if Hue::Winedark as i32 != 21 {
        std::process::exit(4);
    }

    let col = Cell::new(Hue::Claret);
    let cp = &col;

    if cp.get() != Hue::Claret {
        std::process::exit(5);
    }
    if cp.get() == Hue::Burgundy {
        std::process::exit(6);
    }

    if check_vals(cp.get()) != 12 {
        std::process::exit(7);
    }

    col.set(Hue::Winedark);
    if check_vals(cp.get()) != 13 {
        std::process::exit(8);
    }

    std::process::exit(0);
}
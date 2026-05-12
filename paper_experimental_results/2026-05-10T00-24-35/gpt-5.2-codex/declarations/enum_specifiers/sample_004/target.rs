use std::cell::Cell;
use std::process::exit;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    if Hue::Chartreuse as i32 != 0 {
        exit(1);
    }
    if Hue::Burgundy as i32 != 1 {
        exit(2);
    }
    if Hue::Claret as i32 != 20 {
        exit(3);
    }
    if Hue::Winedark as i32 != 21 {
        exit(4);
    }

    let col = Cell::new(Hue::Claret);
    let cp: &Cell<Hue> = &col;

    if cp.get() != Hue::Claret {
        exit(5);
    }
    if cp.get() == Hue::Burgundy {
        exit(6);
    }

    if check_vals(cp.get()) != 12 {
        exit(7);
    }

    col.set(Hue::Winedark);
    if check_vals(cp.get()) != 13 {
        exit(8);
    }

    exit(0);
}
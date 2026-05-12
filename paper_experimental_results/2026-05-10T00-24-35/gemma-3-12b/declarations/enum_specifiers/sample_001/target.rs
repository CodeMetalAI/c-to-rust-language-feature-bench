#[derive(PartialEq, Eq, Clone, Copy)]
enum Hue {
    Chartreuse,
    Burgundy,
    Claret,
    Winedark,
}

impl Hue {
    fn to_usize(self) -> usize {
        match self {
            Hue::Chartreuse => 0,
            Hue::Burgundy => 1,
            Hue::Claret => 20,
            Hue::Winedark => 21,
        }
    }
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
    let chartreuse = Hue::Chartreuse;
    let burgundy = Hue::Burgundy;
    let claret = Hue::Claret;
    let winedark = Hue::Winedark;
    
    if chartreuse.to_usize() != 0 {
        return 1;
    }
    if burgundy.to_usize() != 1 {
        return 2;
    }
    if claret.to_usize() != 20 {
        return 3;
    }
    if winedark.to_usize() != 21 {
        return 4;
    }

    let mut col = Hue::Claret;
    let cp = &col;

    if *cp != Hue::Claret {
        return 5;
    }
    if *cp == Hue::Burgundy {
        return 6;
    }

    if check_vals(*cp) != 12 {
        return 7;
    }

    col = Hue::Winedark;
    if check_vals(*cp) != 13 {
        return 8;
    }

    0
}
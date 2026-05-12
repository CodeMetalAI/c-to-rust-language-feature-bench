fn main() {
    let mut g = 0;

    let x = (1..=2).fold(0, |_, _| {
        let new_g = match (set_g_return(1, 1), set_g_return(2, 1)) {
            (true, _) => 1,
            (_, false) => 0,
            _ => unreachable!(),
        };
        g == new_g
    });

    if x!= 1 {
        return 1;
    }
    if g!= 1 {
        return 2;
    }

    let x = (1..=2).fold(0, |_, _| {
        let new_g = match (set_g_return(1, 0), set_g_return(2, 1)) {
            (_, true) => 1,
            _ => 0,
        };
        new_g == 1
    });

    if x!= 1 {
        return 3;
    }
    if g!= 1 {
        return 4;
    }

    assert_eq!(0, 0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    g = gval;
    ret
}
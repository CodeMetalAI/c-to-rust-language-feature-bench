struct HoldP {
    p: Vec<i32>,
}

struct HoldA {
    tag: i32,
    a: Vec<i32>,
}

fn sum_ptr(p: &[i32]) -> i32 {
    let mut s = 0;
    for &item in p.iter() {
        s += item;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    for &item in a.iter() {
        s += item;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p.iter().sum()
}

fn main() {
    let mut y = vec![1, 2, 3, 4, 5, 6, 7];

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    let mut backing = vec![10, 20, 30];
    let x = &backing;

    if sum_ptr(x) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    if mutate_through_pointer(&mut backing) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if backing[1] != 25 {
        std::process::exit(4);
    }

    if y[6] != 7 {
        std::process::exit(5);
    }

    let hp = HoldP { p: y.clone() };
    if hp.p[0] != 1 {
        std::process::exit(6);
    }

    let ha = HoldA {
        tag: 0,
        a: y.clone(),
    };

    if ha.a[2] != 3 {
        std::process::exit(8);
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
}
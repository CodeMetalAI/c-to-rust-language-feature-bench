use std::mem;

const CALLS: std::cell::RefCell<usize> = std::cell::RefCell::new(0);

fn pos(x: i32) -> i32 {
    *CALLS.borrow_mut() += 1;
    if x <= 0 {
        1
    } else {
        x
    }
}

const FA: [f32; 11] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
];

const AFP: [&mut f32; 17] = [
    &mut 100 as f32,
    &mut 101 as f32,
    &mut 102 as f32,
    &mut 103 as f32,
    &mut 104 as f32,
    &mut 105 as f32,
    &mut 106 as f32,
    &mut 107 as f32,
    &mut 108 as f32,
    &mut 109 as f32,
    &mut 110 as f32,
    &mut 111 as f32,
    &mut 112 as f32,
    &mut 113 as f32,
    &mut 114 as f32,
    &mut 115 as f32,
    &mut 116 as f32,
];

fn init_globals() {
    for i in 0..11 {
        FA[i] = i as f32 + 1.0;
    }
    for i in 0..17 {
        AFP[i] = 100 as f32 + i as f32;
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += p[i] as i32;
    }
    sum
}

fn sum_pointed_ints(pp: &[_], n: usize) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += pp[i] as i32;
    }
    sum
}

fn takes_params(a: &[f32; 11], afp2: &[_]) -> i32 {
    let sum_a = a[0] as i32 + a[10] as i32;
    let sum_afp2 = afp2[0] as i32 + afp2[16] as i32;
    sum_a + sum_afp2
}

fn main() {
    init_globals();

    if FA[0] != 1.0 || FA[10] != 11.0 {
        return 1;
    }

    if AFP[0] as i32 != 100 || AFP[16] as i32 != 116 {
        return 2;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = vec![0.0; n1];
    let mut vla2: Vec<&mut f32> = Vec::new();

    for i in 0..n1 {
        vla1[i] = FA[i] * 2.0;
    }

    for i in 0..n2 {
        vla2.push(AFP[i]);
    }

    if *CALLS.borrow() != 2 {
        return 3;
    }

    let span = (std::mem::size_of::<f32>() * (n1 - 1)) as usize;
    if span != std::mem::size_of_val(&vla1) {
        return 4;
    }

    if sum_ints_from_float(&vla1, 11) != {
        2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22
    } {
        return 5;
    }

    if sum_pointed_ints(&vla2, 17) != {
        100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
        112 + 113 + 114 + 115 + 116
    } {
        return 6;
    }

    if takes_params(&vla1, &vla2) != {
        vla1[0] as i32 + vla1[10] as i32 + vla2[0] as i32 + vla2[16] as i32
    } {
        return 7;
    }
}
struct W {
    a: [i32; 3],
    b: i32,
}

const W_ARRAY: [W; 2] = [
    W { a: [1, 0, 0], b: 0 },
    W { a: [2, 0, 0], b: 0 },
];

fn main() -> i32 {
    if W_ARRAY.len() != 2 {
        return 1;
    }

    if W_ARRAY[0].a[0] != 1 {
        return 2;
    }
    if W_ARRAY[0].a[1] != 0 {
        return 3;
    }
    if W_ARRAY[0].a[2] != 0 {
        return 4;
    }
    if W_ARRAY[0].b != 0 {
        return 5;
    }

    if W_ARRAY[1].a[0] != 2 {
        return 6;
    }
    if W_ARRAY[1].a[1] != 0 {
        return 7;
    }
    if W_ARRAY[1].a[2] != 0 {
        return 8;
    }
    if W_ARRAY[1].b != 0 {
        return 9;
    }

    0
}
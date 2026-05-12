struct S {
    a: [i32; 3],
    b: i32,
}

static W: [S; 2] = [
    S { a: [1, 0, 0], b: 0 },
    S { a: [2, 0, 0], b: 0 },
];

fn main() {
    std::process::exit({
        if W.len() != 2 {
            1
        } else if W[0].a[0] != 1 {
            2
        } else if W[0].a[1] != 0 {
            3
        } else if W[0].a[2] != 0 {
            4
        } else if W[0].b != 0 {
            5
        } else if W[1].a[0] != 2 {
            6
        } else if W[1].a[1] != 0 {
            7
        } else if W[1].a[2] != 0 {
            8
        } else if W[1].b != 0 {
            9
        } else {
            0
        }
    });
}
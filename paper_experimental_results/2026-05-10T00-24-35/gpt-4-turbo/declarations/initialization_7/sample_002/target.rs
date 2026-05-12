fn main() {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if a.len() != 2 {
        std::process::exit(1);
    }
    if b.len() != 3 {
        std::process::exit(2);
    }

    if sum(&a) != 3 {
        std::process::exit(3);
    }
    if sum(&b) != 12 {
        std::process::exit(4);
    }

    if a[1] != 2 {
        std::process::exit(5);
    }
    if b[2] != 5 {
        std::process::exit(6);
    }
}

fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
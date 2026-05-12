fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if a.len()!= 2 {
        return 1;
    }
    if b.len()!= 3 {
        return 2;
    }

    if sum(&a)!= 3 {
        return 3;
    }
    if sum(&b)!= 12 {
        return 4;
    }

    let pa = a.as_ptr();
    let pb = b.as_ptr();
    if unsafe { *pa.offset(1) }!= 2 {
        return 5;
    }
    if unsafe { *pb.offset(2) }!= 5 {
        return 6;
    }

    return 0;
}
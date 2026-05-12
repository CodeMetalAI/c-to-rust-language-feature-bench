struct S1 {
    v1: i32,
    s2p: Option<&'static S2>,
}

struct S2 {
    v2: i32,
    s1p: Option<&'static S1>,
}

fn main() {
    static mut A: S1 = S1 { v1: 0, s2p: None };
    static mut B: S2 = S2 { v2: 0, s1p: None };

    unsafe {
        A.v1 = 10;
        B.v2 = 20;

        A.s2p = Some(&B);
        B.s1p = Some(&A);

        if A.s2p.unwrap().v2!= 20 {
            std::process::exit(1);
        }

        if B.s1p.unwrap().v1!= 10 {
            std::process::exit(2);
        }

        if A.s2p.unwrap().s1p.unwrap() as *const _!= &A as *const _ {
            std::process::exit(3);
        }
    }
}
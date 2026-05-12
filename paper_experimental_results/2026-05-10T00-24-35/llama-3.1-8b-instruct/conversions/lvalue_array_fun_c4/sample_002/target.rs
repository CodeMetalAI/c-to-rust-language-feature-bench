typedef unsigned long size_t;

#[derive(Debug, PartialEq)]
enum TypeId {
    F1,
    F2,
    Default,
}

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if TypeId::from(&p1) != TypeId::F1 {
        println!("p1 of wrong type");
        return 1;
    }
    if TypeId::from(&q1) != TypeId::F1 {
        println!("q1 of wrong type");
        return 2;
    }

    if &p1 != &q1 {
        println!("p1 and q1 are not the same");
        return 3;
    }

    if p1(3) != 4 {
        println!("p1(3) != 4");
        return 4;
    }
    if f1(3) != 4 {
        println!("f1(3) != 4");
        return 5;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if TypeId::from(&r1) != TypeId::F1 {
        println!("r1 of wrong type");
        return 6;
    }
    if r1(4) != 5 {
        println!("r1(4) != 5");
        return 7;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if TypeId::from(&p2) != TypeId::F2 {
        println!("p2 of wrong type");
        return 8;
    }
    if TypeId::from(&q2) != TypeId::F2 {
        println!("q2 of wrong type");
        return 9;
    }

    if &p2 != &q2 {
        println!("p2 and q2 are not the same");
        return 10;
    }
    if p2(2, 3) != 5 {
        println!("p2(2,3) != 5");
        return 11;
    }
    if f2(2, 3) != 5 {
        println!("f2(2,3) != 5");
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if TypeId::from(&r2) != TypeId::F2 {
        println!("r2 of wrong type");
        return 13;
    }
    if r2(10, 20) != 30 {
        println!("r2(10,20) != 30");
        return 14;
    }

    println!("0");
}

impl TypeId {
    fn from(pfn: &dyn Fn(i32) -> i32) -> Self {
        match pfn(0) {
            1 => Self::F1,
            2 => Self::F2,
            _ => Self::Default,
        }
    }

    fn from(pfn: &dyn Fn(i32, i32) -> i32) -> Self {
        match pfn(0, 0) {
            1 => Self::F1,
            2 => Self::F2,
            _ => Self::Default,
        }
    }
}
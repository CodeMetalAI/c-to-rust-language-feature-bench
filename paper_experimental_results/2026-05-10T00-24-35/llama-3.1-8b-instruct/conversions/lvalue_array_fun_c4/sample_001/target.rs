// lvalue_array_fun_rust.rs

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1.id() != 1 {
        println!("Error in f1");
        return;
    }
    if q1.id() != 1 {
        println!("Error in f1");
        return;
    }

    if &p1 as *const _ != &q1 as *const _ {
        println!("Error in f1");
        return;
    }

    if p1(3) != 4 {
        println!("Error in f1");
        return;
    }
    if f1(3) != 4 {
        println!("Error in f1");
        return;
    }

    let r1 = if true { f1 } else { f1 };
    if r1.id() != 1 {
        println!("Error in f1");
        return;
    }
    if r1(4) != 5 {
        println!("Error in f1");
        return;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2.id() != 2 {
        println!("Error in f2");
        return;
    }
    if q2.id() != 2 {
        println!("Error in f2");
        return;
    }

    if &p2 as *const _ != &q2 as *const _ {
        println!("Error in f2");
        return;
    }

    if p2(2, 3) != 5 {
        println!("Error in f2");
        return;
    }
    if f2(2, 3) != 5 {
        println!("Error in f2");
        return;
    }

    let r2 = if false { f2 } else { f2 };
    if r2.id() != 2 {
        println!("Error in f2");
        return;
    }
    if r2(10, 20) != 30 {
        println!("Error in f2");
        return;
    }

    println!("All tests passed");
}

// Define trait for functions with ID (signature)
trait Id<T> {
    fn id(&self) -> usize;
}

// Implement trait for fn
impl<T> Id<T> for fn(T) {
    fn id(&self) -> usize {
        std::any::type_id::<T>()
    }
}
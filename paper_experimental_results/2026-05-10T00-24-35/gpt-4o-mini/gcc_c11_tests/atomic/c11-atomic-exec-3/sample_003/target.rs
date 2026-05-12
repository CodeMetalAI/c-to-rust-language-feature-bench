use std::sync::atomic::{AtomicUsize, Ordering};

fn test_incdec() {
    fn test_incdec_type<T>(value: T, pre_op: fn(&AtomicUsize) -> T, post_op: fn(&AtomicUsize) -> T, pre_p: bool, change: isize) 
    where
        T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::convert::From<isize> + std::cmp::PartialEq,
    {
        let a = AtomicUsize::new(value.into());
        let expected_value = if pre_p {
            value + change.into()
        } else {
            value
        };

        if pre_op(&a) != expected_value {
            std::process::abort();
        }
        if a.load(Ordering::SeqCst) != (value + change.into()) {
            std::process::abort();
        }
    }

    fn test_incdec_arith(value: isize) {
        test_incdec_type(value, |a| {
            a.fetch_add(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        }, |a| a.load(Ordering::SeqCst), true, 1);
        
        test_incdec_type(value, |a| {
            a.fetch_sub(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        }, |a| a.load(Ordering::SeqCst), true, -1);
        
        test_incdec_type(value, |a| a.load(Ordering::SeqCst), |a| {
            a.fetch_add(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        }, false, 1);
        
        test_incdec_type(value, |a| a.load(Ordering::SeqCst), |a| {
            a.fetch_sub(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        }, false, -1);
    }

    test_incdec_arith(0);
    test_incdec_arith(1);
    test_incdec_arith(2);
    test_incdec_arith(-1);
    test_incdec_arith(1 << 60);
    test_incdec_arith(1.5 as isize);

    let mut ia = [0; 2];
    test_incdec_type(&mut ia[1], |a| {
        a.fetch_add(1, Ordering::SeqCst);
        a.load(Ordering::SeqCst)
    }, |a| a.load(Ordering::SeqCst), true, 1);
    
    test_incdec_type(&mut ia[1], |a| {
        a.fetch_sub(1, Ordering::SeqCst);
        a.load(Ordering::SeqCst)
    }, |a| a.load(Ordering::SeqCst), true, -1);

    test_incdec_type(&mut ia[1], |a| a.load(Ordering::SeqCst), |a| {
        a.fetch_add(1, Ordering::SeqCst);
        a.load(Ordering::SeqCst)
    }, false, 1);
    
    test_incdec_type(&mut ia[1], |a| a.load(Ordering::SeqCst), |a| {
        a.fetch_sub(1, Ordering::SeqCst);
        a.load(Ordering::SeqCst)
    }, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}
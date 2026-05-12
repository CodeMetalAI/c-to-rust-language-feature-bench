use std::mem;

static mut BACKING: [i32; 7] = [10, 20, 30, 0, 0, 0, 0];
static mut X: *mut i32 = &mut BACKING[0] as *mut i32;

struct HoldP {
    p: *mut i32;
};

struct HoldA {
    tag: i32;
    a: [i32; 7];
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        unsafe {
            s += *p.offset(i as isize);
        }
    }
    s
}

fn sum_arr(arr: &[i32]) -> i32 {
    let mut s = 0;
    for &val in arr {
        s += val;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
        *p.offset(0 as isize) += 50;
        *p.offset(2)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Initialize y array
        let mut y: [i32; 7] = [1, 2， 3, 4, 5, 6];
        
        // Test sum_arr
        if sum_arr(&y) != 1 + 2 + 3 + 4 + 5 + 6 {
            return Err("sum_arr failed".into());
        }
        
        // Test sum_ptr with x
        let x: *mut i32 = &mut BACKING[0];
        if sum_ptr(x, 3) != 10 + 20 + 30 {
            return Err("sum_ptr failed".into());
        }
        
        // Test mutate_through_pointer
        if mutate_through_pointer(x) != 25 {
            return Err("mutate_through_pointer failed".into());
        }
        
        // Test BACKING was mutated
        if BACKING[1] != 25 {
            return Err("BACKING[1] incorrect".into());
        }
        
        // Test pointer arithmetic with structs
        let mut hp: HoldP = HoldP { p: &mut y[0] };
        if hp.p[0] != 1 {
            return Err("hp.p[0] incorrect".into());
        }
        
        let ha: *mut HoldA = &mut y[0] as *mut HoldA;
        unsafe {
            if (*ha).a[0] != 1 {
                return Err("ha->a[0] incorrect".into());
            }
            
            // Use offsetof equivalent
            let a_ptr: *mut i32 = mem::transmute(
                (ha as *mut u8).offset(mem::offset_of!(HoldA, a) as isize)
            );
            if (*a_ptr) != 1 {
                return Err("offsetof access failed".into());
            }
        }
        
        // Test that y[0] is still 1
        if y[0] != 1 {
            return Err("y[0] changed incorrectly".into());
        }
        
        Ok(())
    }
}

// Helper function to simulate OFFSETOF
#[inline]
fn offset_of<T, U>(_: &T, member: &U) -> usize {
    mem::offset_of!(T, member)
}
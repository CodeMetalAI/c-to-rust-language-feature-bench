static mut C_VP: *const () = std::ptr::null();
static mut VP: *mut () = std::ptr::null_mut();
static mut C_IP: *const i32 = std::ptr::null();
static mut V_IP: *mut i32 = std::ptr::null_mut();
static mut IP: *mut i32 = std::ptr::null_mut();
static mut C_CP: *const i8 = std::ptr::null();

fn f(_p: Option<*const ()>) {}
fn f1(_p: Option<*mut ()>) {}
fn f2(_p: Option<*mut i32>) {}
fn f4(_p: Option<*const i32>) {}
fn f3(_p: Option<*const i32>) {}

fn main() {
    // Using Option to represent nullable pointers in safe Rust
    // The conditional expressions always evaluate to the first branch since condition is always true (1)
    
    let c_vp: Option<*const ()> = None;
    let vp: Option<*mut ()> = None;
    let c_ip: Option<*const i32> = None;
    let v_ip: Option<*mut i32> = None;
    let ip: Option<*mut i32> = None;
    let c_cp: Option<*const i8> = None;

    // f(1 ? c_vp : c_ip) - always takes c_vp branch
    f(if true { c_vp } else { c_ip.map(|p| p as *const ()) });
    // f(1 ? c_ip : c_vp)
    f(if true { c_ip.map(|p| p as *const ()) } else { c_vp });

    // f2(1 ? v_ip : 0)
    f2(if true { v_ip } else { None });
    // f2(1 ? 0 : v_ip)
    f2(if true { None } else { v_ip });

    // f3(1 ? c_ip : v_ip)
    f3(if true { c_ip } else { v_ip.map(|p| p as *const i32) });
    // f3(1 ? v_ip : c_ip)
    f3(if true { v_ip.map(|p| p as *const i32) } else { c_ip });

    // f(1 ? vp : c_cp)
    f(if true { vp.map(|p| p as *const ()) } else { c_cp.map(|p| p as *const ()) });
    // f(1 ? c_cp : vp)
    f(if true { c_cp.map(|p| p as *const ()) } else { vp.map(|p| p as *const ()) });

    // f4(1 ? ip : c_ip)
    f4(if true { ip.map(|p| p as *const i32) } else { c_ip });
    // f4(1 ? c_ip : ip)
    f4(if true { c_ip } else { ip.map(|p| p as *const i32) });

    // f1(1 ? vp : ip)
    f1(if true { vp } else { ip.map(|p| p as *mut ()) });
    // f1(1 ? ip : vp)
    f1(if true { ip.map(|p| p as *mut ()) } else { vp });
}
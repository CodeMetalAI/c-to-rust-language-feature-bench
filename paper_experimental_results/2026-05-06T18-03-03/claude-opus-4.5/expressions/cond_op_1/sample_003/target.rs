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
    let c_vp: Option<*const ()> = Some(std::ptr::null());
    let vp: Option<*mut ()> = Some(std::ptr::null_mut());
    let c_ip: Option<*const i32> = Some(std::ptr::null());
    let v_ip: Option<*mut i32> = Some(std::ptr::null_mut());
    let ip: Option<*mut i32> = Some(std::ptr::null_mut());
    let c_cp: Option<*const i8> = Some(std::ptr::null());

    f(if true { c_vp } else { c_ip.map(|p| p as *const ()) });
    f(if true { c_ip.map(|p| p as *const ()) } else { c_vp });

    f2(if true { v_ip } else { None });
    f2(if true { None } else { v_ip });

    f3(if true { c_ip } else { v_ip.map(|p| p as *const i32) });
    f3(if true { v_ip.map(|p| p as *const i32) } else { c_ip });

    f(if true { vp.map(|p| p as *const ()) } else { c_cp.map(|p| p as *const ()) });
    f(if true { c_cp.map(|p| p as *const ()) } else { vp.map(|p| p as *const ()) });

    f4(if true { ip.map(|p| p as *const i32) } else { c_ip });
    f4(if true { c_ip } else { ip.map(|p| p as *const i32) });

    f1(if true { vp } else { ip.map(|p| p as *mut ()) });
    f1(if true { ip.map(|p| p as *mut ()) } else { vp });
}
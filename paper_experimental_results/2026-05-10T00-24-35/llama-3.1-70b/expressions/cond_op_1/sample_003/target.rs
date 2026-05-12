fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *mut i32) {}
fn f3(_p: *const i32) {}
fn f4(_p: *const i32) {}

fn main() {
    static C_VP: *const () = 0 as *const ();
    static VP: *mut () = 0 as *mut ();
    static C_IP: *const i32 = 0 as *const i32;
    static V_IP: *mut i32 = 0 as *mut i32;
    static IP: *mut i32 = 0 as *mut i32;
    static C_CP: *const u8 = 0 as *const u8;

    f(if 1 { C_VP } else { C_IP });
    f(if 1 { C_IP } else { C_VP });

    f2(if 1 { V_IP } else { std::ptr::null_mut() });
    f2(if 1 { std::ptr::null_mut() } else { V_IP });

    f3(if 1 { C_IP } else { V_IP as *const i32 });
    f3(if 1 { V_IP as *const i32 } else { C_IP });

    f(if 1 { VP } else { C_CP as *const () });
    f(if 1 { C_CP as *const () } else { VP });

    f4(if 1 { IP } else { C_IP });
    f4(if 1 { C_IP } else { IP });

    f1(if 1 { VP } else { IP });
    f1(if 1 { IP } else { VP });
}
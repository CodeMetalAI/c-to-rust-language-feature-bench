use std::any::Any;

fn f(_p: Option<&dyn Any>) {}

fn f1(_p: Option<&mut dyn Any>) {}

fn f2(_p: Option<&mut dyn Any>) {}

fn f4(_p: Option<&dyn Any>) {}

fn f3(_p: Option<&dyn Any>) {}

fn main() {
    let mut vp_val = ();
    let mut ip_val = 0i32;
    let mut v_ip_val = 0i32;
    let c_vp: &dyn Any = &vp_val;
    let vp: &mut dyn Any = &mut vp_val;
    let c_ip: &dyn Any = &ip_val;
    let v_ip: &mut dyn Any = &mut v_ip_val;
    let ip: &mut dyn Any = &mut ip_val;
    let c_cp: &dyn Any = &"";

    f(if true { Some(c_vp) } else { Some(c_ip) });
    f(if true { Some(c_ip) } else { Some(c_vp) });

    f2(if true { Some(v_ip) } else { None });
    f2(if true { None } else { Some(v_ip) });

    f3(if true { Some(c_ip) } else { Some(v_ip as &dyn Any) });
    f3(if true { Some(v_ip as &dyn Any) } else { Some(c_ip) });

    f(if true { Some(vp as &dyn Any) } else { Some(c_cp) });
    f(if true { Some(c_cp) } else { Some(vp as &dyn Any) });

    f4(if true { Some(ip as &dyn Any) } else { Some(c_ip) });
    f4(if true { Some(c_ip) } else { Some(ip as &dyn Any) });

    f1(if true { Some(vp) } else { Some(ip) });
    f1(if true { Some(ip) } else { Some(vp) });
}
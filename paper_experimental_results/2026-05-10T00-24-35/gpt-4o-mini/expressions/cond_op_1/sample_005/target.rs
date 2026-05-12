/*
 * cond_op_1
 */

const C_VP: Option<&'static ()> = None;
let mut vp: Option<&mut ()> = None;
const C_IP: Option<&'static i32> = None;
let mut v_ip: Option<&mut i32> = None;
let mut ip: Option<&mut i32> = None;
const C_CP: Option<&'static str> = None;

fn f(p: Option<&()>) { let _ = p; }
fn f1(p: Option<&mut ()>) { let _ = p; }
fn f2(p: Option<&mut i32>) { let _ = p; }
fn f4(p: Option<&i32>) { let _ = p; }
fn f3(p: Option<&mut i32>) { let _ = p; }

fn main() {
    f(Some(&())); // 1 ? C_VP : C_IP
    f(C_IP); // 1 ? C_IP : C_VP

    f2(v_ip); // 1 ? V_IP : 0
    f2(None); // 1 ? 0 : V_IP

    f3(C_IP); // 1 ? C_IP : V_IP
    f3(v_ip); // 1 ? V_IP : C_IP

    f(Some(&())); // 1 ? VP : C_CP
    f(C_CP); // 1 ? C_CP : VP

    f4(ip); // 1 ? IP : C_IP
    f4(C_IP); // 1 ? C_IP : IP

    f1(vp); // 1 ? VP : IP
    f1(ip); // 1 ? IP : VP
}
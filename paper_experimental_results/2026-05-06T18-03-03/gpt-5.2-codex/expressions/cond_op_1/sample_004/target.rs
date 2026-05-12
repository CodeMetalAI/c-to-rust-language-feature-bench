type ConstVoidPtr = usize;
type VoidPtr = usize;
type ConstIntPtr = usize;
type VolatileIntPtr = usize;
type IntPtr = usize;
type ConstCharPtr = usize;

static C_VP: ConstVoidPtr = 0;
static VP: VoidPtr = 0;
static C_IP: ConstIntPtr = 0;
static V_IP: VolatileIntPtr = 0;
static IP: IntPtr = 0;
static C_CP: ConstCharPtr = 0;

fn f(_p: ConstVoidPtr) {}
fn f1(_p: VoidPtr) {}
fn f2(_p: VolatileIntPtr) {}
fn f4(_p: ConstIntPtr) {}
fn f3(_p: ConstIntPtr) {}

fn main() {
    f(if true { C_VP } else { C_IP });
    f(if true { C_IP } else { C_VP });

    f2(if true { V_IP } else { 0 });
    f2(if true { 0 } else { V_IP });

    f3(if true { C_IP } else { V_IP });
    f3(if true { V_IP } else { C_IP });

    f(if true { VP } else { C_CP });
    f(if true { C_CP } else { VP });

    f4(if true { IP } else { C_IP });
    f4(if true { C_IP } else { IP });

    f1(if true { VP } else { IP });
    f1(if true { IP } else { VP });
}
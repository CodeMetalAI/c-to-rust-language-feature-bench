type ConstVoidPtr = Option<()>;
type VoidPtr = Option<()>;
type ConstIntPtr = Option<()>;
type VolatileIntPtr = Option<()>;
type IntPtr = Option<()>;
type ConstCharPtr = Option<()>;
type ConstVolatileIntPtr = Option<()>;

static C_VP: ConstVoidPtr = None;
static VP: VoidPtr = None;
static C_IP: ConstIntPtr = None;
static V_IP: VolatileIntPtr = None;
static IP: IntPtr = None;
static C_CP: ConstCharPtr = None;

fn f(_p: ConstVoidPtr) {}
fn f1(_p: VoidPtr) {}
fn f2(_p: VolatileIntPtr) {}
fn f4(_p: ConstIntPtr) {}
fn f3(_p: ConstVolatileIntPtr) {}

fn main() {
    f(if true { C_VP } else { C_IP });
    f(if true { C_IP } else { C_VP });

    f2(if true { V_IP } else { None });
    f2(if true { None } else { V_IP });

    f3(if true { C_IP } else { V_IP });
    f3(if true { V_IP } else { C_IP });

    f(if true { VP } else { C_CP });
    f(if true { C_CP } else { VP });

    f4(if true { IP } else { C_IP });
    f4(if true { C_IP } else { IP });

    f1(if true { VP } else { IP });
    f1(if true { IP } else { VP });
}
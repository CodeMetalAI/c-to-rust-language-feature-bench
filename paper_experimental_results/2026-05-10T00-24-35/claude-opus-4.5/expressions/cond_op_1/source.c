/*
 * cond_op_1
 */

const void *c_vp;
void *vp;
const int *c_ip;
volatile int *v_ip;
int *ip;
const char *c_cp;

static void f(const void *p) { (void)p; }
static void f1(void *p) { (void)p; }
static void f2(volatile int *p) { (void)p; }
static void f4(const int *p) { (void)p; }
static void f3(const volatile int *p) { (void)p; }

int main(void) {
  f(1 ? c_vp : c_ip);
  f(1 ? c_ip : c_vp);

  f2(1 ? v_ip : 0);
  f2(1 ? 0 : v_ip);

  f3(1 ? c_ip : v_ip);
  f3(1 ? v_ip : c_ip);

  f(1 ? vp : c_cp);
  f(1 ? c_cp : vp);

  f4(1 ? ip : c_ip);
  f4(1 ? c_ip : ip);

  f1(1 ? vp : ip);
  f1(1 ? ip : vp);

  return 0;
}

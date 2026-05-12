/* external_objs_1.c
 */

int i1 = 1;
static int i2 = 2;
extern int i3 = 3;

int i4;
static int i5;

int i1;
static int i2;
int i3;
int i4;
static int i5;

extern int i1;
extern int i3;
extern int i4;

static int *p_i1 = &i1;
static int *p_i2 = &i2;
static int *p_i4 = &i4;
static int *p_i5 = &i5;

int main(void) {
  if (i1 != 1)
    return 1;
  if (i2 != 2)
    return 2;
  if (i3 != 3)
    return 3;

  if (i4 != 0)
    return 4;
  if (i5 != 0)
    return 5;

  if (p_i1 != &i1)
    return 6;
  if (p_i2 != &i2)
    return 7;
  if (p_i4 != &i4)
    return 8;
  if (p_i5 != &i5)
    return 9;

  i1 = 10;
  i2 = 20;
  i4 = 30;
  i5 = 40;

  if (*p_i1 != 10)
    return 10;
  if (*p_i2 != 20)
    return 11;
  if (*p_i4 != 30)
    return 12;
  if (*p_i5 != 40)
    return 13;

  return 0;
}

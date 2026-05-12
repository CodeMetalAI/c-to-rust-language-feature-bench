/*
 * goto_2.c
 */

int main(void) {
  int n = 4;
  int j = 1;

  int saw_lab3 = 0;
  int saw_lab4 = 0;

  int out = 0;

  {
    double a[n];

    a[j] = 4.4;
    out += 44;

    goto lab3;

  lab3:
    saw_lab3 = 1;
    a[j] = 3.3;
    out += 33;

    goto lab4;

    a[j] = 5.5;

  lab4:
    saw_lab4 = 1;
    a[j] = 6.6;
    out += 66;
  }

  if (!saw_lab3)
    return 2;
  if (!saw_lab4)
    return 3;
  if (out != 143)
    return 4;

  return 0;
}

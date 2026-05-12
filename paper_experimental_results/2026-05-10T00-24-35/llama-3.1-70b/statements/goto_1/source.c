/*
 * goto_1.c
 */
int main(void) {

  int acc = 0;
  goto first_time;
  for (int i = 0; i < 5; i++) {
    acc++;
    if (acc < 15) {
      acc = -15;
    }

  first_time:
    acc = 15;
    continue;
  }

  return acc == 20 ? 0 : 1;
}

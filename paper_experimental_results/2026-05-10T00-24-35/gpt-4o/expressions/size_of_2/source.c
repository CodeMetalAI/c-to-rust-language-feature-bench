/*
 * size_of_2
 */

int main(void) {
  int a[5] = {1, 2, 3, 4, 5};

  int n = (int)(sizeof a / sizeof a[0]);

  if (n != 5)
    return 1;

  return 0;
}

// initialization_10.c
typedef struct {
  int quot;
  int rem;
} div_t;

int main(void) {
  div_t answer = {.quot = 2, .rem = -1};

  if (answer.quot != 2)
    return 1;
  if (answer.rem != -1)
    return 2;

  {
    div_t swapped = {.rem = -3, .quot = 4};
    if (swapped.quot != 4)
      return 3;
    if (swapped.rem != -3)
      return 4;
  }

  return 0;
}

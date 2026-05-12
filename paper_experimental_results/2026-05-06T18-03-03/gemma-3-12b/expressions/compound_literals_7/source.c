/*
 * compound_literal_7
 */

struct int_list {
  int car;
  struct int_list *cdr;
};

static struct int_list endless_zeros = {0, &endless_zeros};

static int eval(struct int_list x) {
  if (x.car != 0)
    return 1;
  if (x.cdr != &endless_zeros)
    return 2;
  if (x.cdr->cdr != &endless_zeros)
    return 3;
  return 0;
}

int main(void) { return eval(endless_zeros); }

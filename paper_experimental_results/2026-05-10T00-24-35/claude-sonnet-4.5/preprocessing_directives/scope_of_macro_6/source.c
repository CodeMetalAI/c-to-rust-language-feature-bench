/* scope_of_macro_6.c */
#define OBJ_LIKE (1 - 1)
#define OBJ_LIKE (1 - 1)

#define FUNC_LIKE(a) (a)
#define FUNC_LIKE(a) (a)

#ifdef NEGATIVE
#define OBJ_LIKE (0)
#define OBJ_LIKE (1e - 1)
#define FUNC_LIKE(b) (a)
#define FUNC_LIKE(b) (b)
#endif

int main(void) {
  int x = OBJ_LIKE;
  int y = FUNC_LIKE(3);

  if (x != 0)
    return 1;
  if (y != 3)
    return 2;

  return 0;
}

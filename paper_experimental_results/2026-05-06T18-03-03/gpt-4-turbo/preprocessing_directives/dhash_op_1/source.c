/* dhash_op_1.c
 */

#define hash_hash # ## #
#define mkstr(a) #a
#define in_between(a) mkstr(a)
#define join(c, d) in_between(c hash_hash d)

char p[] = join(x, y);

int main(void) {
  if (sizeof(p) != 7)
    return 1;
  if (p[0] != 'x')
    return 2;
  if (p[1] != ' ')
    return 3;
  if (p[2] != '#')
    return 4;
  if (p[3] != '#')
    return 5;
  if (p[4] != ' ')
    return 6;
  if (p[5] != 'y')
    return 7;
  if (p[6] != '\0')
    return 8;
  return 0;
}

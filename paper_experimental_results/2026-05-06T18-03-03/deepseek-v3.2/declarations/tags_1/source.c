/*
 * C11 6.7.2.3 — Tags (Example 1)
 */

#include <stdlib.h>

struct tnode {
  int count;
  struct tnode *left;
  struct tnode *right;
};

int main(void) {
  struct tnode s;
  struct tnode *sp;

  struct tnode left_child;
  struct tnode right_child;

  s.count = 1;
  left_child.count = 2;
  right_child.count = 3;

  s.left = &left_child;
  s.right = &right_child;

  sp = &s;

  if (sp->left->count != 2)
    return 1;

  if (s.right->count != 3)
    return 2;

  if (sp->left == sp->right)
    return 3;

  return 0;
}

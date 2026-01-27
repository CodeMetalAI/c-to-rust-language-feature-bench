/*
 * C11 6.7.3 — Type Qualifiers (Example 2)
 *
 * Positive test derived from Example 2.
 * Tests valid uses of const-qualified aggregate types.
 */

#include <stddef.h>

struct s { int mem; };

typedef int A[2][3];

int main(void)
{
    const struct s cs = { 1 };
    struct s ncs;

    const A a = { {4, 5, 6}, {7, 8, 9} };

    int *pi;
    const int *pci;

    /* Assignment from const to non-const object is allowed */
    ncs = cs;

    /* Taking address of non-const member */
    pi = &ncs.mem;

    /* Taking address of const-qualified member */
    pci = &cs.mem;

    /* Reading from const-qualified array element */
    pci = a[0];

    /* Observable correctness */
    if (ncs.mem != 1)
        return 1;

    if (*pci != 4)
        return 2;

    return 0;   /* PASS */
}
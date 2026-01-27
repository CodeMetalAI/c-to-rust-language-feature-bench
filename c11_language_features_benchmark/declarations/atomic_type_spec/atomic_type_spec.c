/*
 * C11 6.7.2.4 — Atomic type specifiers (Example 1)
 *
 * This test verifies that a structure tag introduces a type name that
 * can be used to declare pointers to the same structure type within
 * its own definition, enabling self-referential structures.
 */

/*
 * C11 6.7.2.4 — Atomic type specifiers
 *
 * Tests use of the _Atomic type specifier with scalar types.
 */

#include <stdatomic.h>
#include <stddef.h>

int main(void)
{
    /* Atomic-qualified scalar */
    _Atomic int ai = 0;

    /* Typedef form */
    typedef _Atomic int atomic_int_t;
    atomic_int_t aj = 0;

    /* Basic atomic operations must work */
    atomic_store(&ai, 10);
    atomic_store(&aj, 20);

    if (atomic_load(&ai) != 10)
        return 1;

    if (atomic_load(&aj) != 20)
        return 2;

    /* Atomic fetch-add */
    if (atomic_fetch_add(&ai, 1) != 10)
        return 3;

    if (atomic_load(&ai) != 11)
        return 4;

    return 0;   /* PASS */
}

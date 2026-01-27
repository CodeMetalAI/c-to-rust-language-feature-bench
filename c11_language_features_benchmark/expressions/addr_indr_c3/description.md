**Test name**: addr_indr_c3

**Description:**
Validates C11 §6.5.3.2 address and indirection operators. The test checks that the unary `&` operator
yields the address of its operand, that `&(*p)` is equivalent to `p`, and that `&(a[i])` is equivalent
to `a + i`. These cases demonstrate that certain operator combinations are not evaluated as separate
operations but instead yield the same pointer result.

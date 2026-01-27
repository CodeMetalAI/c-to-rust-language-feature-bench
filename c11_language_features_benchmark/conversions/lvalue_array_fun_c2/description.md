**Test name**: lvalue_array_fun_c2

**Description:**
Clause 2 coverage for 6.3.2.1.
Lvalue conversion yields the stored value except in contexts like sizeof, `\_Alignof`, `unary &`, `++/--`,
member/assignment operands, and it drops qualifiers (and atomicity) on the produced rvalue.

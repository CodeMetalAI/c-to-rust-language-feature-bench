**Test name**: type_def_2

**Description:**
Section 6.7.8 (Type definitions), Example 2 coverage.
Defines `t1`/`tp1` from `struct s1` and `t2`/`tp2` from `struct s2` and forces compatibility checks via well-typed calls: values of type `t1` are passed to functions expecting `t1` and `struct s1` (and vice versa), and a `tp1` pointer is used to mutate a `t1` object through `->`, showing the pointed-to type is compatible with `t1`/`struct s1`. The test also constructs `t2`/`tp2` independently to emphasize that identical-looking `struct s2` is not the same type as `struct s1`/`t1`, while keeping the program strictly positive and defined.

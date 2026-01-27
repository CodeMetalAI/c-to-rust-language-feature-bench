**Test name**: restrict_def_1

**Description:**
Coverage for Section 6.7.3.1 (restrict), exercising Example 1.
Uses file-scope `int * restrict a; int * restrict b; extern int c[];` and assigns `a`/`b` through a union-based "view"
selector that makes it tempting to treat the same underlying storage as accessible via multiple names. The test remains
defined by ensuring `a`, `b`, and `c` designate disjoint objects during the block execution and by accessing/modifying
each object only through its designated restricted path.

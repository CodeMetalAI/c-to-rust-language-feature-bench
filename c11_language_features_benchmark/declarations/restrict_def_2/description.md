**Test name**: restrict_def_2

**Description:**
Section 6.7.3.1 (restrict), Example 2 coverage.
Calls `f(int n, int * restrict p, int * restrict q)` with `p` and `q` derived from the same base array but pointing to disjoint element ranges, asserting that during each execution the objects accessed via `p` are not also accessed via `q`. The test stresses translation by making the non-aliasing guarantee depend on a runtime offset and by validating that only the destination range is updated while the source range and adjacent sentinels remain unchanged.

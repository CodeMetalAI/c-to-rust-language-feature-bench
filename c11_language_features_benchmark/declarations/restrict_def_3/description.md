**Test name**: restrict_def_3

**Description:**
Section 6.7.3.1 (restrict), Example 3 coverage.
Calls `h(n, int * restrict p, int * restrict q, int * restrict r)` in the defined-aliasing pattern `h(n, a, b, b)`, where `q` and `r` alias the same unmodified array `b` while `p` designates a disjoint destination. The test stresses translation by using the same base pointer for two restricted parameters (tempting an incorrect "must not alias" assumption) and verifies both that the destination becomes `2*b` and that `b` remains byte-for-byte unchanged.

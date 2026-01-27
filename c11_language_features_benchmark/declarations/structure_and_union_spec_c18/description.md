**Test name**: structure_and_union_spec_c18

**Description:**
Clause 18 coverage for Section 6.7.2.1.
Exercises flexible array members: `sizeof(struct)` ignores the flexible member, the member's offset is fixed, and `.`/`->` access to the flexible member behaves as if it were replaced by the longest same-typed array that fits in the accessed object. Also covers the zero-element case safely by naming the member without element access or one-past pointer formation.

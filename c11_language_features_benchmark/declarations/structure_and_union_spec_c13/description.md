**Test name**: structure_and_union_spec_c13

**Description:**
Clause 13 coverage for Section 6.7.2.1.
Members of anonymous structures/unions are treated as members of the containing aggregate, and this flattening applies recursively through chains of anonymous structs/unions. The test requires direct access (expressions, addresses, and designated initializers) to names defined several anonymous levels deep.

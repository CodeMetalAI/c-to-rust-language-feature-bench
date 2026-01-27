**Test name**: type_qualif_2

**Description:**
Section 6.7.3 (Example 2) coverage.
Exercises qualifiers on aggregates: a `const`-qualified structure object can be copied into a modifiable structure object (`ncs = cs`), taking `int *` to a member is valid only for the non-const object (`&ncs.mem`), taking `const int *` to a member of the const object is valid (`&cs.mem`), and a `const`-qualified multidimensional array (`const A`) makes its elements `const int`, allowing read-only indexing through both dimensions.

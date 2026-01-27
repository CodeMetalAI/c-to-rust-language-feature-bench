**Test name**: storage_duration_c6

**Description:**
Clause 6 coverage for Section 6.2.4.
An automatic object without static storage is created on each block entry (including recursive entries),
initialized each time its declaration is reached if an initializer is present, and otherwise has an indeterminate
value on each entry.

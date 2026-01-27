**Test name**: restrict_def_4b

**Description:**
Section 6.7.3.1 (restrict), Example 4b coverage.
Exercises the exception that permits carrying the value of a restricted pointer out of its declaring block by returning a struct containing a `float * restrict` field. The test stresses translation by returning two such vectors, verifying their pointers remain usable after return, are distinct allocations, and support independent restricted access in subsequent computations.

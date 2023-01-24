# p4sym, a solver for the connect-4 (puissance 4 in french) game

This program is quite "state of the art" regarding the use of symmetries 
and hash tables. 

It can solve the connect-4 game up to 7x7 given enough memory for the hash tables.
This Rust version is not as fast as the C version I wrote for my students at ISAE/Sup aéro. However, the difference in this version is less than 15% when using a bit of unsafe code in the evaluation function (the 7x6 version in C with the same size for hashtables compiled with clang12 runs in 378s). In fact, the difference seems to come mainly from bounds checking that can not be turned off in Rust, except when using ```get_unchecked()``` which is unsafe.

Parallelism is not used in the main branch. Working on it in the parallel branch.


Results and timings are presented below.


| X | Y |Result|Time|NB_BITS|
|---|---|------|----:|---:|
|2 |4 | Draw | <1s| |
|2 |5 | Draw | <1s| |
|2 |6 | Draw | <1s| |
|2 |7 | Draw | <1s| |
|3 |4 | Draw | <1s| |
|3 |5 | Draw | <1s| |
|3 |6 | Draw | <1s| |
|3 |7 | Draw | <1s| |
|4 |2 | Draw | <1s| |
|4 |3 | Draw | <1s| |
|4 |4 | Draw | <1s| |
|4 |5 | Draw | <1s| |
|4 |6 | Draw | <1s| |
|4 |7 | Draw | <1s| |
|5 |2 | Draw | <1s| |
|5 |3 | Draw | <1s| |
|5 |4 | Draw | <1s| |
|5 |5 | Draw | <1s| |
|5 |6 | Draw | <1s| |
|5 |7 | Draw | 5s |28|
|6 |2 | Draw | <1s| |
|6 |3 | Draw | <1s| |
|6 |4 | Black | <1s| |
|6 |5 | Draw | <1s| |
|6 |6 | Black | 24s |28|
|6 |7 | White | 196s |30|
|7 |2 | Draw | <1s| |
|7 |3 | Draw | <1s| |
|7 |4 | Draw | <1s | |
|7 |5 | Draw | 8s |28|
|7 |6 | White |429s |32|
|7 |7 | Draw |  | |

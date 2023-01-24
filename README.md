# p4sym, a solver for the connect-4 (puissance 4 in french) game

This program is quite "state of the art" regarding the use of symmetries 
and hash tables. 

It can solve the connect-4 game up to 7x7 given enough memory for the hash tables.
This Rust version is not as fast as the C version I wrote for my students at ISAE/Sup aéro. The difference in this version is aroung 10 to 20%.

Solving the "classical" 7x6 game takes around 7 minutes on an i9-9900K.
Parallelism is not used in the main branch. Working on it in the parallel branch.


Results and timings are presented below.


| X | Y |Result|Time (in seconds)|NB_BITS|
|---|---|------|----|--- |
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
|5 |7 | Draw | 4.675093529 |28|
|6 |2 | Draw | <1s| |
|6 |3 | Draw | <1s| |
|6 |4 | Black | <1s| |
|6 |5 | Draw | <1s| |
|6 |6 | Black | 23.976995134 |28|
|6 |7 | White | 195.763339926 |30|
|7 |2 | Draw | <1s| |
|7 |3 | Draw | <1s| |
|7 |4 | Draw | <1s | |
|7 |5 | Draw | 8.398944051 |28|
|7 |6 | White |428.865146357 |32|
|7 |7 | Draw |  | |

# p4sym, a solver for the connect-4 (puissance 4 in french) game

This program is quite "state of the art" regarding the use of symmetries 
and hash tables. 

It can solve the connect-4 game up to 7x7 given enough memory for the hash tables.
This Rust version is as fast as the C version I wrote for my students at ISAE/Sup aéro.

Solving the "classical" 7x6 game takes around 20 minutes on an i9-9900K, the 7x7 grid takes around 40 hours.
Parallelism is not used.

The program works only in "release" mode (cargo run --release)

Results and timings are presented below.


| X | Y |Result|Time|
|---|---|------|----|
|1 |1 | Draw | 0.000002 |
|1 |2 | Draw | 0.000001 |
|1 |3 | Draw | 0.000002 |
|1 |4 | Draw | 0.000001 |
|1 |5 | Draw | 0.000002 |
|1 |6 | Draw | 0.000003 |
|1 |7 | Draw | 0.000003 |
|2 |1 | Draw | 0.000002 |
|2 |2 | Draw | 0.000003 |
|2 |3 | Draw | 0.000004 |
|2 |4 | Draw | 0.000007 |
|2 |5 | Draw | 0.000011 |
|2 |6 | Draw | 0.000013 |
|2 |7 | Draw | 0.000025 |
|3 |1 | Draw | 0.000004 |
|3 |2 | Draw | 0.000005 |
|3 |3 | Draw | 0.000018 |
|3 |4 | Draw | 0.000032 |
|3 |5 | Draw | 0.000067 |
|3 |6 | Draw | 0.000141 |
|3 |7 | Draw | 0.000311 |
|4 |1 | Draw | 0.000007 |
|4 |2 | Draw | 0.000015 |
|4 |3 | Draw | 0.000074 |
|4 |4 | Draw | 0.001124 |
|4 |5 | Draw | 0.004161 |
|4 |6 | Draw | 0.051274 |
|4 |7 | Draw | 0.123396 |
|5 |1 | Draw | 0.000004 |
|5 |2 | Draw | 0.000042 |
|5 |3 | Draw | 0.000361 |
|5 |4 | Draw | 0.005164 |
|5 |5 | Draw | 0.083063 |
|5 |6 | Draw | 0.403507 |
|5 |7 | Draw | 5.602671 |
|6 |1 | Draw | 0.000010 |
|6 |2 | Draw | 0.000135 |
|6 |3 | Draw | 0.002643 |
|6 |4 | Black | 0.099493 |
|6 |5 | Draw | 0.854485 |
|6 |6 | Black | 27.831886 |
|6 |7 | White | 485.194268 |
|7 |1 | Draw | 0.000025 |
|7 |2 | Draw | 0.000434 |
|7 |3 | Draw | 0.012567 |
|7 |4 | Draw | 1.039251 |
|7 |5 | Draw | 10.346796 |
|7 |6 | White | 1368.662978 |
|7 |7 | Draw | 153151.122663 |

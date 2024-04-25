# Selecting the `i'th largest of n` elements

| n  | k | forward search (single threaded) | backward search (multi threaded) | bidirection search (alt) |
| -  | - | -         | -            | -          |
| 12 | 0 | ?         | 0.0s         | ?          |
| 12 | 1 | ?         | 0.1s         | ?          |
| 12 | 2 | ?         | 0.4s         | ?          |
| 12 | 3 | ?         | 1.6s         | ?          |
| 12 | 4 | ?         | 9.8s         | ?          |
| 12 | 5 | ?         | 57.1s        | ?          |
| -  | - | -         | -            | -          |
| 13 | 0 | ?         | 0.0s         | ?          |
| 13 | 1 | ?         | 0.2s         | ?          |
| 13 | 2 | ?         | 1.1s         | ?          |
| 13 | 3 | ?         | 19.1s        | ?          |
| 13 | 4 | ?         | 2m 19.6s     | ?          |
| 13 | 5 | ?         | 12m 27.8s    | ?          |
| 13 | 6 | ?         | 26m 36.8s    | ?          |
| -  | - | -         | -            | -          |
| 14 | 0 | ?         | 0.0s         | ?          |
| 14 | 1 | ?         | 0.6s         | ?          |
| 14 | 2 | ?         | 4.6s         | ?          |
| 14 | 3 | ?         | 55.3s        | ?          |
| 14 | 4 | ?         | 21m 50.6s    | ?          |
| 14 | 5 | ?         | 2h 22m 57.2s | ?          |
| 14 | 6 | ?         | ?            | ?          |

(alles Stand 25.04.2024; Hardware: Julius Laptop -> da kein Server verfügbar)

## Table of Contents

## Introduction

The following introduction was copied from [1]. Our motivation was to confirm the values in the
table, find optimal algorithms for values where only a lower bound exists and add some values
for `n=16`.

This page gives algorithms for finding the `i'th largest of 'n` elements with the fewest possible
comparisons.

The problem has been widely studied in literature. **Knuth**[2] gives a brief history of the problem and
some known upper and lower bounds. Optimal algorithms are known for any `n' when i` is one or two,
but for example for finding the median (i = n/2), there is a significant performance gap from the
best known algorithm `(approximately 2.97 n + o(n) comparisons)` to the tightest known minimum
`(2 n - o(n))`.

Many other special cases are also known, such as the ones given in the table below. Many of these
were previously found by **Gasarch, Kelly and Pugh**[3] who introduced computer searching to find the
optimal selection algorithms. The additional results presented in the table above have been
found by **my independent implementation**[4], but it probably shares many characteristics with Gasarch's.

Numbers marked with one asterisk are new lower bounds. Numbers marked with two asterisks improve
previously known algorithms. Since I found no previous studies of `V(14,3)` and `V(15,3)` in the
literature, I have marked it with a question mark.

Click the number to get the optimal selection algorithm. If an optimal algorithm is not known,
I've given the tightest known lower bound and the best known algorithm.

| V(n,i) | i = 1        | i = 2        | i = 3          | i = 4            | i = 5            | i = 6            | i = 7                    | i = 8  |
| ------ | ------------ | ------------ | -------------- | ---------------- | ---------------- | ---------------- | ------------------------ | ------ |
| n = 2  | [1](V_2_1)   |              |                |                  |                  |                  |                          |        |
| n = 3  | [2](V_3_1)   | [3](V_3_2)   |                |                  |                  |                  |                          |        |
| n = 4  | [3](V_4_1)   | [4](V_4_2)   |                |                  |                  |                  |                          |        |
| n = 5  | [4](V_5_1)   | [6](V_5_2)   | [6](V_5_3)     |                  |                  |                  |                          |        |
| n = 6  | [5](V_6_1)   | [7](V_6_2)   | [8](V_6_3)     |                  |                  |                  |                          |        |
| n = 7  | [6](V_7_1)   | [8](V_7_2)   | [10](V_7_3)    | [10](V_7_4)      |                  |                  |                          |        |
| n = 8  | [7](V_8_1)   | [9](V_8_2)   | [11](V_8_3)    | [12](V_8_4)      |                  |                  |                          |        |
| n = 9  | [8](V_9_1)   | [11](V_9_2)  | [12](V_9_3)    | [14](V_9_4)      | [14](V_9_5)      |                  |                          |        |
| n = 10 | [9](V_10_1)  | [12](V_10_2) | [14](V_10_3)   | [15](V_10_4)     | [16](V_10_5)     |                  |                          |        |
| n = 11 | [10](V_11_1) | [13](V_11_2) | [15](V_11_3)   | [17](V_11_4)     | [18](V_11_5)\*   | [18](V_11_6)\*\* |                          |        |
| n = 12 | [11](V_12_1) | [14](V_12_2) | [17](V_12_3)   | [18](V_12_4)\*\* | [19](V_12_5)\*\* | [20](V_12_6)\*\* |                          |        |
| n = 13 | [12](V_13_1) | [15](V_13_2) | [18](V_13_3)\* | [20](V_13_4)\*   | [21](V_13_5)\*\* | [22](V_13_6)\*\* | [23](V_13_7)\*           |        |
| n = 14 | [13](V_14_1) | [16](V_14_2) | [19](V_14_3)?  | [21](V_14_4)\*   | [23](V_14_5)\*   | [24](V_14_6)\*\* | 24\*(..[25](V_14_7)\*\*) |        |
| n = 15 | [14](V_15_1) | [17](V_15_2) | [20](V_15_3)?  | [23](V_15_4)\*   | [25](V_15_5)\*   | 25\*(..26)       | 23..28                   | 24..28 |

## Improved known optimal compares

| V(n,i) | i = 1        | i = 2        | i = 3         | i = 4          | i = 5              | i = 6            | i = 7                    | i = 8  |
| ------ | ------------ | ------------ | ------------- | -------------- | ------------------ | ---------------- | ------------------------ | ------ |
| n = 14 | [13](V_14_1) | [16](V_14_2) | [19](V_14_3)? | [21](V_14_4)\* | [23](V_14_5)\*     | [24](V_14_6)\*\* | 24\*(..[25](V_14_7)\*\*) |        |
| n = 15 | [14](V_15_1) | [17](V_15_2) | [20](V_15_3)? | [23](V_15_4)\* | [**24**](V_15_5)\* | 25\*(..26)       | 23..28                   | 24..28 |
| n = 16 | [14](V_16_1) | [17](V_16_2) | [20](V_16_3)  | [23](V_16_4)   | [25](V_16_5)       | 25\*(..26)       | 23..28                   | 24..28 |

## Requirements

Since the project depends on the rust crate nauty, a C compiler able to compile it is required in addition to Rust. 

To install `Rust` please follow the guide on the **website**[5].

The package version used to create the data is

```shell
$ rustc -V
rustc 1.77.0-nightly (6ae4cfbbb 2024-01-17)
$ pacman -Q nauty
nauty 1:2.8.8-2
$ pacman -Q clang
clang 16.0.6-1
```

## Quick start

Having all required packages installed, starting a calculation on your own is straight forward.

```shell
cargo run --release -- -n 10 -i 3
```

This will start calculating compares starting at the **i=4th** (index starts at 0) value for a list of **n=10** and will continue calculating until you stop.

For more details please read

```shell
./target/release/selection_generator --help
```

## Calculation times

One can find the calculation times in the `times*.txt` files. The times were generated using the hardware listed in the []

## Hardware used

The hardware used to calculate these results was

```shell
<TBD>
```

## References

[1]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/](https://www.cs.hut.fi/~cessu/selection/)

[2]: Knuth. _Research_. [https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3](https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3)

[3]: Gasarch, Kelly and Pugh. _Research_. [https://www.cs.umd.edu/~gasarch/papers/select.ps](https://www.cs.umd.edu/~gasarch/papers/select.ps)

[4]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/selgen.c.html](https://www.cs.hut.fi/~cessu/selection/selgen.c.html)

[5]: Rust [https://www.rust-lang.org/](https://www.rust-lang.org/)

[6]: Nauty [https://pallini.di.uniroma1.it/](https://pallini.di.uniroma1.it/)

[7]: Clang [https://clang.llvm.org/](https://clang.llvm.org/)

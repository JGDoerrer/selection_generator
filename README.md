# Finding Lower Bounds for the Number of Comparison in Selection Algorithms

## Introduction

This research project aims to find worst case optimal comparison algorithms for selecting the `i`-th smallest of `n` elements of a set for `n` up to `15` with computer search.


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

To calculate all values from the start, you can use :

```shell
cargo run --release -- --search-mode forward --verbose --print-algorithm

cargo run --release -- --search-mode backward --verbose --max-core 16 --print-algorithm
```

To calculate a specific value, e.g. `n = 12`, `i = 5` you can use:

```shell
cargo run --release -- --search-mode forward --verbose --print-algorithm --single -n 12 -i 5

cargo run --release -- --search-mode backward --verbose --max-core 16 --print-algorithm --single -n 12 -i 5
```

This will start calculating compares starting at the **i=4th** (index starts at 0) value for a list of **n=10** and will continue calculating until you stop.

For more details please read

```shell
./target/release/selection_generator --help
```


## Results

The minimum amount of comparisons needed to select the `i`-th smallest of `n`

| `n` \ `i` | 0  | 1  | 2  | 3  | 4  | 5  | 6  | 7  |
| -         | -  | -  | -  | -  | -  | -  | -  | -  |
| 1         | 0  |    |    |    |    |    |    |    |
| 2         | 1  |    |    |    |    |    |    |    |
| 3         | 2  | 3  |    |    |    |    |    |    |
| 4         | 3  | 4  |    |    |    |    |    |    |
| 5         | 4  | 6  | 6  |    |    |    |    |    |
| 6         | 5  | 7  | 8  |    |    |    |    |    |
| 7         | 6  | 8  | 10 | 10 |    |    |    |    |
| 8         | 7  | 9  | 11 | 12 |    |    |    |    |
| 9         | 8  | 11 | 12 | 14 | 14 |    |    |    |
| 10        | 9  | 12 | 14 | 15 | 16 |    |    |    |
| 11        | 10 | 13 | 15 | 17 | 18 | 18 |    |    |
| 12        | 11 | 14 | 17 | 18 | 19 | 20 |    |    |
| 13        | 12 | 15 | 18 | 20 | 21 | 22 | 23 |    |
| 14        | 13 | 16 | 19 | 21 | 23 | 24 | 25 |    |
| 15        | 14 | 17 | 20 | 23 | 24 | 26 | 26 | 27 |

Comparison of the times for the forward and backward search (note: the forward search runs single threaded, in contrast to the backward search which benefits greatly from parallelism; the forward search was started with 500gb RAM)

| `n` | `i` | forward search   | backward search |
| -   | -   | -                | -               |
| 12  | 0   | 0.0s             | 0.0s            |
| 12  | 1   | 0.0s             | 0.1s            |
| 12  | 2   | 0.4s             | 0.7s            |
| 12  | 3   | 3.5s             | 1.6s            |
| 12  | 4   | 36.1s            | 8.4s            |
| 12  | 5   | 1m 29.9s         | 42.0s           |
| -   | -   | -                | -               |
| 13  | 0   | 0.0s             | 0.0s            |
| 13  | 1   | 0.0s             | 0.5s            |
| 13  | 2   | 0.8s             | 1.5s            |
| 13  | 3   | 13.8s            | 16.1s           |
| 13  | 4   | 3m 41.9s         | 1m 40.2s        |
| 13  | 5   | 17m 9.9s         | 8m 34.7s        |
| 13  | 6   | 59m 19.8s        | 18m 4.8s        |
| -   | -   | -                | -               |
| 14  | 0   | 0.0s             | 0.0s            |
| 14  | 1   | 0.0s             | 1.5s            |
| 14  | 2   | 1.4s             | 5.9s            |
| 14  | 3   | 35.9s            | 46.9s           |
| 14  | 4   | 17m 27.2s        | 15m 32.5s       |
| 14  | 5   | 2h 39m 49.5s     | 1h 39m 59.6s    |
| 14  | 6   | 14h 39m 41.5s    | 6h 26m 30.8s    |
| -   | -   | -                | -               |
| 15  | 0   | 0.0s             | 0.0s            |
| 15  | 1   | 0.1s             | 4.0s            |
| 15  | 2   | 2.8s             | 25.9s           |
| 15  | 3   | 2m 23.7s         | 13m 10.9s       |
| 15  | 4   | 1h 12m 9.6s      | 45m 51.9s       |
| 15  | 5   | 1d 8h 37m        | 19h 30m 20.6s   |
| 15  | 6   | 12d 13h 43m      | 1d 5h 42m 51.5s |
| 15  | 7   | 14d 1h 51m       | 3d 8h 8m 32.6s  |


## Hardware used

For hardware, we employed two Intel Xeon CPUs, each equipped with 12 cores (24 threads), and a total of 768 GB of RAM.


## References

[1]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/](https://www.cs.hut.fi/~cessu/selection/)

[2]: Knuth. _Research_. [https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3](https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3)

[3]: Gasarch, Kelly and Pugh. _Research_. [https://www.cs.umd.edu/~gasarch/papers/select.ps](https://www.cs.umd.edu/~gasarch/papers/select.ps)

[4]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/selgen.c.html](https://www.cs.hut.fi/~cessu/selection/selgen.c.html)

[5]: Rust [https://www.rust-lang.org/](https://www.rust-lang.org/)

[6]: Nauty [https://pallini.di.uniroma1.it/](https://pallini.di.uniroma1.it/)

[7]: Clang [https://clang.llvm.org/](https://clang.llvm.org/)

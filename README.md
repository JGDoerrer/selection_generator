# Finding Lower Bounds for the Number of Comparison in Selection Algorithms

## Introduction

This research project aims to find worst case optimal comparison algorithms for selecting the $i$-th smallest of $n$ elements of a set for $n$ up to $15$ with computer search.
Explicitly, we apply computer search to determine the optimal worst-case number of comparisons for selecting a single element from a set of initially unordered elements.
Our method comprises the three main approaches forward search, backward search and bidirectional search.
Additionally, we harness the notion of compatible solutions in all of the three searches.
For the forward search approach besides pruning, poset canonification and heuristics, multithreading was applied successfully.
In backward search the greatest challenge is posed by the task of calculating the predecessors.
This was tackled by a three-step procedure that can be performed within reasonable time and storage costs.

Time measurements showed that the backward search performed slightly faster for smaller $n$ and $i$.
For higher values, up to $n = 15$ and $i = 8$ the backward search consumed only a fraction of the time taken by the forward search.
Overall, we could not only confirm solutions of previous research but also improve the lower bound for $n = 15$ and $i = 5$ by one comparison.
Furthermore, we can disprove the conjecture that ``pair-forming algorithms'', in which the first comparison of any singleton with another singleton is performed, do not lead to suboptimal results.
For $n = 12$ and $i = 5$, this assumption gives a bound of $20$ comparisons, which does not correspond to the optimal bound of $19$ comparisons.


## Requirements

Since the project depends on the rust crate nauty, a `C`-compiler able to compile it is required in addition to Rust.

To install `rust` please follow the guide on the **website**[5].

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
Note: In contrast to the paper, in the program $i$ starts at $0$. Please be aware that also all log files (except the readme) also start with $i = 0$.

To calculate all values from the start, you can use:

```shell
cargo run --release -- --search-mode forward --verbose --print-algorithm

cargo run --release -- --search-mode backward --verbose --max-core 16 --print-algorithm
```

To calculate a specific value, e.g. the value `n = 12, i = 6` from the paper, the program must be started with the arguments `-n 12 -i 5`:

```shell
cargo run --release -- --search-mode forward --verbose --print-algorithm --single -n 12 -i 5

cargo run --release -- --search-mode backward --verbose --max-core 16 --print-algorithm --single -n 12 -i 5
```

For more details please read

```shell
./target/release/selection_generator --help
```

## Verify Test

To verify the produced algorithms for a produced algorithm `12_5.rs` you have to replace it with the file `algorithm.rs` and execute the test:

```shell
cargo test algorithm_test --release
```

You can also automatically test all algorithm in the folder `./algorithms` with:

```shell
sh test_algorithms.sh
```

We attached our algorithms from the backward-search in `./logs/backward/algorithms`.


## Results

The minimum amount of comparisons needed to select the $i$-th smallest of $n$

| $n$ \ $i$ | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  |
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
| 16        | 15 | 18 | 21 | 24 | 26 | 27 |  ? |  ? |

Comparison of the times for the forward and backward search (note: the forward search runs single threaded, in contrast to the backward search which benefits greatly from parallelism; the forward search was started with 500gb RAM)

| $n$ | $i$ | forward search | backward search |
| -   | -   | -              | -               |
| 12  | 1   | 0.0s           | 0.0s            |
| 12  | 2   | 0.0s           | 0.2s            |
| 12  | 3   | 0.4s           | 0.6s            |
| 12  | 4   | 3.5s           | 0.9s            |
| 12  | 5   | 36.1s          | 3.8s            |
| 12  | 6   | 1m 30s         | 18.0s           |
| -   | -   | -              | -               |
| 13  | 1   | 0.0s           | 0.0s            |
| 13  | 2   | 0.0s           | 0.5s            |
| 13  | 3   | 0.8s           | 1.2s            |
| 13  | 4   | 13.8s          | 10.3s           |
| 13  | 5   | 3m 42s         | 44.5s           |
| 13  | 6   | 17m 10s        | 3m 22s          |
| 13  | 7   | 59m 20s        | 7m 16s          |
| -   | -   | -              | -               |
| 14  | 1   | 0.0s           | 0.0s            |
| 14  | 2   | 0.0s           | 1.3s            |
| 14  | 3   | 1.4s           | 5.1s            |
| 14  | 4   | 35.9s          | 33.0s           |
| 14  | 5   | 17m 27s        | 7m 1s           |
| 14  | 6   | 2h 40m         | 37m 54s         |
| 14  | 7   | 14h 40m        | 2h 17m          |
| -   | -   | -              | -               |
| 15  | 1   | 0.0s           | 0.0s            |
| 15  | 2   | 0.1s           | 3.9s            |
| 15  | 3   | 2.8s           | 24.5s           |
| 15  | 4   | 2m 24s         | 11m 2s          |
| 15  | 5   | 1h 12m         | 22m 11s         |
| 15  | 6   | 1d 8h 37m      | 7h 17m          |
| 15  | 7   | 4d 23h 37m     | 9h 45m          |
| 15  | 8   | 14d 1h 51m     | 1d 3h 7m        |
| -   | -   | -              | -               |
| 16  | 1   | -              | 0.0s            |
| 16  | 2   | -              | 12.3s           |
| 16  | 3   | -              | 1m 55.1s        |
| 16  | 4   | -              | 52m 9.4s        |
| 16  | 5   | -              | 6h 48m 14.8s    |
| 16  | 6   | -              | 1d 1h 26m       |
| 16  | 7   | -              | ?               |
| 16  | 8   | -              | ?               |


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

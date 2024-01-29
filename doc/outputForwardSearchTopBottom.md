# Program Output (search from Top to Bottom)
```
time '0.000ns' + '0.000ns' = '0.000ns': n = 2, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 1

time '0.000ns' + '0.000ns' = '0.000ns': n = 3, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 2
time '0.000ns' + '0.000ns' = '0.000ns': n = 3, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 3

time '0.000ns' + '0.000ns' = '0.000ns': n = 4, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 3
time '0.000ns' + '0.000ns' = '0.000ns': n = 4, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 4

time '0.000ns' + '0.000ns' = '0.000ns': n = 5, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 4
time '0.000ns' + '0.000ns' = '0.000ns': n = 5, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 6
time '0.000ns' + '0.000ns' = '0.000ns': n = 5, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 6

time '0.000ns' + '0.000ns' = '0.000ns': n = 6, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 5
time '0.000ns' + '0.000ns' = '0.000ns': n = 6, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 7
time '0.000ns' + '0.000ns' = '0.000ns': n = 6, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 8

time '0.000ns' + '0.000ns' = '0.000ns': n = 7, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 6
time '0.000ns' + '0.000ns' = '0.000ns': n = 7, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1 + 0 = 1, comparisons: 8
# search with Pair-Optimisation & maxComparisons = 9 -> NoSolution in 1.441ms
# search with maxComparisons = 9 -> NoSolution in 924.250µs
time '1.441ms' + '924.250µs' = '2.366ms': n = 7, i = 2, (cache_l: 1444, cache_u: 598, noSol: 3, bruteForce: 174), cache = 42 + 125 = 167, comparisons: 10
# search with Pair-Optimisation & maxComparisons = 11 -> FoundSolution in 45.084µs
# search with Pair-Optimisation & maxComparisons = 10 -> FoundSolution in 86.075µs
# search with Pair-Optimisation & maxComparisons = 9 -> NoSolution in 968.581µs
# search with maxComparisons = 9 -> NoSolution in 761.818µs
time '1.100ms' + '761.818µs' = '1.862ms': n = 7, i = 3, (cache_l: 1189, cache_u: 441, noSol: 0, bruteForce: 107), cache = 59 + 213 = 272, comparisons: 10

time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 59 + 213 = 272, comparisons: 7
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 59 + 213 = 272, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 59 + 213 = 272, comparisons: 11
# search with Pair-Optimisation & maxComparisons = 12 -> FoundSolution in 67.840µs
# search with Pair-Optimisation & maxComparisons = 11 -> NoSolution in 5.222ms
# search with maxComparisons = 11 -> NoSolution in 12.014ms
time '5.290ms' + '12.014ms' = '17.305ms': n = 8, i = 3, (cache_l: 10000, cache_u: 2993, noSol: 0, bruteForce: 686), cache = 169 + 694 = 863, comparisons: 12

time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 169 + 694 = 863, comparisons: 8
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 169 + 694 = 863, comparisons: 11
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 169 + 694 = 863, comparisons: 12
# search with Pair-Optimisation & maxComparisons = 13 -> NoSolution in 102.495ms
# search with maxComparisons = 13 -> NoSolution in 59.099ms
time '102.495ms' + '59.099ms' = '161.594ms': n = 9, i = 3, (cache_l: 78856, cache_u: 20916, noSol: 2, bruteForce: 3986), cache = 666 + 3434 = 4100, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 15 -> FoundSolution in 168.178µs
# search with Pair-Optimisation & maxComparisons = 14 -> FoundSolution in 27.870ms
# search with Pair-Optimisation & maxComparisons = 13 -> NoSolution in 95.158ms
# search with maxComparisons = 13 -> NoSolution in 67.957ms
time '123.196ms' + '67.957ms' = '191.153ms': n = 9, i = 4, (cache_l: 98397, cache_u: 24221, noSol: 0, bruteForce: 4587), cache = 993 + 6957 = 7950, comparisons: 14

time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 993 + 6957 = 7950, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 993 + 6957 = 7950, comparisons: 12
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 993 + 6957 = 7950, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 14 -> NoSolution in 81.780ms
# search with maxComparisons = 14 -> NoSolution in 283.023ms
time '81.780ms' + '283.023ms' = '364.802ms': n = 10, i = 3, (cache_l: 146068, cache_u: 27014, noSol: 0, bruteForce: 5185), cache = 1312 + 11146 = 12458, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 16 -> FoundSolution in 475.871µs
# search with Pair-Optimisation & maxComparisons = 15 -> NoSolution in 748.836ms
# search with maxComparisons = 15 -> NoSolution in 1.699s
time '749.312ms' + '1.699s' = '2.449s': n = 10, i = 4, (cache_l: 996745, cache_u: 204156, noSol: 3, bruteForce: 37505), cache = 3824 + 37145 = 40969, comparisons: 16

time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 3824 + 37145 = 40969, comparisons: 10
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 3824 + 37145 = 40969, comparisons: 13
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 3824 + 37145 = 40969, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 18 -> FoundSolution in 95.812ms
# search with Pair-Optimisation & maxComparisons = 17 -> FoundSolution in 701.361ms
# search with Pair-Optimisation & maxComparisons = 16 -> NoSolution in 2.108s
# search with maxComparisons = 16 -> NoSolution in 2.585s
time '2.905s' + '2.585s' = '5.490s': n = 11, i = 3, (cache_l: 1614349, cache_u: 273500, noSol: 6, bruteForce: 47877), cache = 7132 + 73286 = 80418, comparisons: 17
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 22.918s
# search with maxComparisons = 17 -> NoSolution in 10.991s
time '22.918s' + '10.991s' = '33.909s': n = 11, i = 4, (cache_l: 9011048, cache_u: 1607039, noSol: 42, bruteForce: 269163), cache = 21974 + 259226 = 281200, comparisons: 18
# search with Pair-Optimisation & maxComparisons = 19 -> FoundSolution in 517.012µs
# search with Pair-Optimisation & maxComparisons = 18 -> FoundSolution in 15.605s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 15.555s
# search with maxComparisons = 17 -> NoSolution in 12.745s
time '31.161s' + '12.745s' = '43.906s': n = 11, i = 5, (cache_l: 12575250, cache_u: 2115349, noSol: 10, bruteForce: 367488), cache = 35518 + 536514 = 572032, comparisons: 18
```
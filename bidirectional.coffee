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
# search with Pair-Optimisation & maxComparisons = 9# 1: 1 -> ? => ? -> 1 in 41.484µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 178.627µs | total cached: 5
 -> NoSolution in 2.178ms
# search with maxComparisons = 9# 3: 3 -> ? => ? -> 14 in 2.248ms | total cached: 19
 -> NoSolution in 2.953ms
time '2.178ms' + '2.953ms' = '5.131ms': n = 7, i = 2, (cache_l: 669, cache_u: 231, noSol: 0, bruteForce: 139), cache = 35 + 105 = 140, comparisons: 10
# search with Pair-Optimisation & maxComparisons = 11 -> FoundSolution in 72.784µs
# search with Pair-Optimisation & maxComparisons = 10# 1: 1 -> ? => ? -> 1 in 46.559µs | total cached: 2
 -> FoundSolution in 83.583µs
# search with Pair-Optimisation & maxComparisons = 9# 2: 1 -> ? => ? -> 3 in 178.192µs | total cached: 5
 -> NoSolution in 1.539ms
# search with maxComparisons = 9# 3: 3 -> ? => ? -> 12 in 1.615ms | total cached: 17
 -> NoSolution in 2.722ms
time '1.695ms' + '2.722ms' = '4.417ms': n = 7, i = 3, (cache_l: 367, cache_u: 139, noSol: 0, bruteForce: 68), cache = 48 + 160 = 208, comparisons: 10

time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 160 = 208, comparisons: 7
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 160 = 208, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 160 = 208, comparisons: 11
# search with Pair-Optimisation & maxComparisons = 12# 1: 1 -> ? => ? -> 1 in 45.740µs | total cached: 2
 -> FoundSolution in 137.385µs
# search with Pair-Optimisation & maxComparisons = 11# 2: 1 -> ? => ? -> 3 in 171.776µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 4.645ms | total cached: 21
 -> NoSolution in 7.657ms
# search with maxComparisons = 11# 4: 16 -> ? => ? -> 93 in 27.762ms | total cached: 114
 -> NoSolution in 34.580ms
time '7.795ms' + '34.580ms' = '42.375ms': n = 8, i = 3, (cache_l: 3425, cache_u: 819, noSol: 0, bruteForce: 458), cache = 131 + 535 = 666, comparisons: 12

time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 131 + 535 = 666, comparisons: 8
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 131 + 535 = 666, comparisons: 11
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 131 + 535 = 666, comparisons: 12
# search with Pair-Optimisation & maxComparisons = 13# 1: 1 -> ? => ? -> 1 in 37.871µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 180.209µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 8.232ms | total cached: 21
# 4: 16 -> ? => ? -> 137 in 93.535ms | total cached: 158
 -> NoSolution in 228.461ms
# search with maxComparisons = 13 -> NoSolution in 183.229ms
time '228.461ms' + '183.229ms' = '411.690ms': n = 9, i = 3, (cache_l: 27343, cache_u: 4962, noSol: 0, bruteForce: 2618), cache = 548 + 2736 = 3284, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 15# 1: 1 -> ? => ? -> 1 in 46.872µs | total cached: 2
 -> FoundSolution in 156.840µs
# search with Pair-Optimisation & maxComparisons = 14# 2: 1 -> ? => ? -> 3 in 168.304µs | total cached: 5
 -> FoundSolution in 407.580µs
# search with Pair-Optimisation & maxComparisons = 13# 3: 3 -> ? => ? -> 16 in 5.505ms | total cached: 21
# 4: 16 -> ? => ? -> 119 in 60.667ms | total cached: 140
 -> NoSolution in 171.290ms
# search with maxComparisons = 13# 5: 119 -> ? => ? -> 643 in 204.565ms | total cached: 783
 -> NoSolution in 155.324ms
time '171.854ms' + '155.324ms' = '327.178ms': n = 9, i = 4, (cache_l: 14575, cache_u: 2564, noSol: 0, bruteForce: 1487), cache = 719 + 4052 = 4771, comparisons: 14

time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 719 + 4052 = 4771, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 719 + 4052 = 4771, comparisons: 12
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 719 + 4052 = 4771, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 14# 1: 1 -> ? => ? -> 1 in 36.093µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 165.710µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 6.624ms | total cached: 21
# 4: 16 -> ? => ? -> 164 in 184.920ms | total cached: 185
 -> NoSolution in 193.246ms
# search with maxComparisons = 14 -> NoSolution in 766.925ms
time '193.246ms' + '766.925ms' = '960.172ms': n = 10, i = 3, (cache_l: 47154, cache_u: 7449, noSol: 0, bruteForce: 3255), cache = 1033 + 6993 = 8026, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 16# 1: 1 -> ? => ? -> 1 in 45.999µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 167.976µs | total cached: 5
 -> FoundSolution in 871.580µs
# search with Pair-Optimisation & maxComparisons = 15# 3: 3 -> ? => ? -> 16 in 7.390ms | total cached: 21
# 4: 16 -> ? => ? -> 216 in 213.517ms | total cached: 237
 -> NoSolution in 1.845s
# search with maxComparisons = 15# 5: 216 -> ? => ? -> 2372 in 2.252s | total cached: 2609
 -> NoSolution in 4.279s
time '1.846s' + '4.279s' = '6.124s': n = 10, i = 4, (cache_l: 184366, cache_u: 29590, noSol: 0, bruteForce: 16162), cache = 2720 + 21468 = 24188, comparisons: 16

time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 2720 + 21468 = 24188, comparisons: 10
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 2720 + 21468 = 24188, comparisons: 13
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 2720 + 21468 = 24188, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 18# 1: 1 -> ? => ? -> 1 in 46.731µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 169.144µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 7.305ms | total cached: 21
# 4: 16 -> ? => ? -> 174 in 365.116ms | total cached: 195
 -> FoundSolution in 1.131s
# search with Pair-Optimisation & maxComparisons = 17 -> FoundSolution in 3.791s
# search with Pair-Optimisation & maxComparisons = 16# 5: 174 -> ? => ? -> 1730 in 4.645s | total cached: 1925
 -> NoSolution in 2.806s
# search with maxComparisons = 16 -> NoSolution in 6.357s
time '7.728s' + '6.357s' = '14.085s': n = 11, i = 3, (cache_l: 491143, cache_u: 83250, noSol: 0, bruteForce: 26660), cache = 6017 + 44831 = 50848, comparisons: 17
# search with Pair-Optimisation & maxComparisons = 17# 1: 1 -> ? => ? -> 1 in 56.343µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 200.578µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 8.696ms | total cached: 21
# 4: 16 -> ? => ? -> 283 in 568.065ms | total cached: 304
# 5: 283 -> ? => ? -> 4736 in 10.498s | total cached: 5040
 -> NoSolution in 40.486s
# search with maxComparisons = 17 -> NoSolution in 32.243s
time '40.486s' + '32.243s' = '72.729s': n = 11, i = 4, (cache_l: 2097027, cache_u: 314886, noSol: 0, bruteForce: 125021), cache = 18653 + 157216 = 175869, comparisons: 18
# search with Pair-Optimisation & maxComparisons = 19# 1: 1 -> ? => ? -> 1 in 48.754µs | total cached: 2
# 2: 1 -> ? => ? -> 3 in 168.295µs | total cached: 5
# 3: 3 -> ? => ? -> 16 in 6.985ms | total cached: 21
 -> FoundSolution in 279.491ms
# search with Pair-Optimisation & maxComparisons = 18# 4: 16 -> ? => ? -> 247 in 434.261ms | total cached: 268
# 5: 247 -> ? => ? -> 3870 in 6.457s | total cached: 4138
 -> FoundSolution in 14.063s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 15.724s
# search with maxComparisons = 17# 6: 3870 -> ? => ? -> 32394 in 43.400s | total cached: 36532
 -> NoSolution in 26.718s
time '30.067s' + '26.718s' = '56.785s': n = 11, i = 5, (cache_l: 1305387, cache_u: 192522, noSol: 0, bruteForce: 84125), cache = 25599 + 234395 = 259994, comparisons: 18
# Program Output (bidirectional search with backward search multi threaded)
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
# search with Pair-Optimisation & maxComparisons = 9# 1: 1 => 1 in 16.724µs ~ 16.077µs | total cached: 2
# 2: 1 => 2 in 26.190µs ~ 193.791µs | total cached: 5
 -> NoSolution in 1.292ms
# search with maxComparisons = 9# 3: 3 => 15 in 84.972µs ~ 940.475µs | total cached: 19
# 4: 14 => 41 in 351.054µs ~ 1.243ms | total cached: 59
 -> NoSolution in 1.915ms
time '1.292ms' + '1.915ms' = '3.207ms': n = 7, i = 2, (cache_l: 679, cache_u: 217, noSol: 0, bruteForce: 146), cache = 36 + 111 = 147, comparisons: 10
# search with Pair-Optimisation & maxComparisons = 11 -> FoundSolution in 108.076µs
# search with Pair-Optimisation & maxComparisons = 10# 1: 1 => 1 in 21.086µs ~ 16.554µs | total cached: 90
 -> FoundSolution in 127.358µs
# search with Pair-Optimisation & maxComparisons = 9# 2: 1 => 2 in 23.086µs ~ 204.887µs | total cached: 90
# 3: 3 => 11 in 74.046µs ~ 697.011µs | total cached: 90
 -> NoSolution in 1.091ms
# search with maxComparisons = 9# 4: 12 => 45 in 241.462µs ~ 1.035ms | total cached: 113
 -> NoSolution in 1.279ms
time '1.326ms' + '1.279ms' = '2.605ms': n = 7, i = 3, (cache_l: 337, cache_u: 79, noSol: 0, bruteForce: 69), cache = 48 + 168 = 216, comparisons: 10

time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 168 = 216, comparisons: 7
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 168 = 216, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 8, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 48 + 168 = 216, comparisons: 11
# search with Pair-Optimisation & maxComparisons = 12# 1: 1 => 1 in 35.301µs ~ 33.127µs | total cached: 136
 -> FoundSolution in 109.999µs
# search with Pair-Optimisation & maxComparisons = 11# 2: 1 => 2 in 48.138µs ~ 289.046µs | total cached: 136
# 3: 3 => 21 in 218.856µs ~ 864.067µs | total cached: 136
 -> NoSolution in 4.456ms
# search with maxComparisons = 11# 4: 14 => 100 in 831.610µs ~ 2.315ms | total cached: 160
# 5: 86 => 265 in 1.435ms ~ 3.955ms | total cached: 412
 -> NoSolution in 7.052ms
time '4.566ms' + '7.052ms' = '11.618ms': n = 8, i = 3, (cache_l: 1430, cache_u: 298, noSol: 0, bruteForce: 262), cache = 101 + 377 = 478, comparisons: 12

time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 101 + 377 = 478, comparisons: 8
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 101 + 377 = 478, comparisons: 11
time '0.000ns' + '0.000ns' = '0.000ns': n = 9, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 101 + 377 = 478, comparisons: 12
# search with Pair-Optimisation & maxComparisons = 13# 1: 1 => 1 in 25.837µs ~ 29.247µs | total cached: 624
# 2: 1 => 3 in 53.065µs ~ 367.928µs | total cached: 624
# 3: 3 => 26 in 321.225µs ~ 1.469ms | total cached: 624
# 4: 14 => 201 in 1.699ms ~ 4.526ms | total cached: 650
# 5: 115 => 790 in 9.545ms ~ 15.133ms | total cached: 996
 -> NoSolution in 45.769ms
# search with maxComparisons = 13# 6: 651 => 1963 in 14.827ms ~ 32.358ms | total cached: 2555
 -> NoSolution in 47.010ms
time '45.769ms' + '47.010ms' = '92.779ms': n = 9, i = 3, (cache_l: 6508, cache_u: 1282, noSol: 0, bruteForce: 1162), cache = 325 + 1315 = 1640, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 15# 1: 1 => 1 in 61.428µs ~ 62.980µs | total cached: 3551
 -> FoundSolution in 368.340µs
# search with Pair-Optimisation & maxComparisons = 14# 2: 1 => 2 in 56.593µs ~ 355.620µs | total cached: 3551
# 3: 3 => 27 in 236.526µs ~ 1.279ms | total cached: 3551
# 4: 14 => 183 in 1.615ms ~ 5.086ms | total cached: 3557
# 5: 94 => 833 in 5.078ms ~ 14.835ms | total cached: 3926
 -> FoundSolution in 30.770ms
# search with Pair-Optimisation & maxComparisons = 13 -> NoSolution in 10.390ms
# search with maxComparisons = 13# 6: 686 => 2242 in 14.399ms ~ 34.342ms | total cached: 6026
 -> NoSolution in 47.323ms
time '41.528ms' + '47.323ms' = '88.852ms': n = 9, i = 4, (cache_l: 5871, cache_u: 1069, noSol: 0, bruteForce: 1031), cache = 526 + 2145 = 2671, comparisons: 14

time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 526 + 2145 = 2671, comparisons: 9
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 526 + 2145 = 2671, comparisons: 12
time '0.000ns' + '0.000ns' = '0.000ns': n = 10, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 526 + 2145 = 2671, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 14# 1: 1 => 1 in 52.817µs ~ 70.687µs | total cached: 7143
# 2: 1 => 3 in 114.813µs ~ 414.201µs | total cached: 7143
# 3: 3 => 36 in 632.497µs ~ 1.416ms | total cached: 7143
# 4: 14 => 360 in 4.177ms ~ 7.077ms | total cached: 7157
 -> NoSolution in 67.092ms
# search with maxComparisons = 14# 5: 131 => 2063 in 23.389ms ~ 42.316ms | total cached: 7489
 -> NoSolution in 144.157ms
time '67.092ms' + '144.157ms' = '211.250ms': n = 10, i = 3, (cache_l: 15778, cache_u: 2000, noSol: 0, bruteForce: 1934), cache = 718 + 3887 = 4605, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 16# 1: 1 => 1 in 40.654µs ~ 49.861µs | total cached: 9497
# 2: 1 => 3 in 70.849µs ~ 260.600µs | total cached: 9497
 -> FoundSolution in 545.245µs
# search with Pair-Optimisation & maxComparisons = 15# 3: 3 => 40 in 416.704µs ~ 2.120ms | total cached: 9497
# 4: 15 => 379 in 5.096ms ~ 8.686ms | total cached: 9501
# 5: 126 => 2506 in 19.638ms ~ 54.594ms | total cached: 9954
 -> NoSolution in 171.703ms
# search with maxComparisons = 15# 6: 1497 => 10169 in 94.617ms ~ 214.940ms | total cached: 16663
 -> NoSolution in 574.219ms
time '172.248ms' + '574.219ms' = '746.467ms': n = 10, i = 4, (cache_l: 48732, cache_u: 8619, noSol: 0, bruteForce: 7095), cache = 1796 + 9904 = 11700, comparisons: 16

time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1796 + 9904 = 11700, comparisons: 10
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1796 + 9904 = 11700, comparisons: 13
time '0.000ns' + '0.000ns' = '0.000ns': n = 11, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 1796 + 9904 = 11700, comparisons: 15
# search with Pair-Optimisation & maxComparisons = 18# 1: 1 => 1 in 63.089µs ~ 48.817µs | total cached: 35386
# 2: 1 => 4 in 157.698µs ~ 345.432µs | total cached: 35386
# 3: 3 => 50 in 1.237ms ~ 1.929ms | total cached: 35386
# 4: 14 => 594 in 12.851ms ~ 15.563ms | total cached: 35392
 -> FoundSolution in 176.194ms
# search with Pair-Optimisation & maxComparisons = 17# 5: 139 => 4684 in 82.006ms ~ 138.234ms | total cached: 35653
 -> FoundSolution in 354.440ms
# search with Pair-Optimisation & maxComparisons = 16# 6: 1267 => 23591 in 488.652ms ~ 613.468ms | total cached: 40502
 -> NoSolution in 883.184ms
# search with maxComparisons = 16 -> NoSolution in 827.154ms
time '1.414s' + '827.154ms' = '2.241s': n = 11, i = 3, (cache_l: 180438, cache_u: 25820, noSol: 0, bruteForce: 16359), cache = 4161 + 23898 = 28059, comparisons: 17
# search with Pair-Optimisation & maxComparisons = 17# 1: 1 => 1 in 75.149µs ~ 62.482µs | total cached: 49205
# 2: 1 => 3 in 97.765µs ~ 537.565µs | total cached: 49205
# 3: 3 => 53 in 1.007ms ~ 5.676ms | total cached: 49205
# 4: 15 => 674 in 16.428ms ~ 27.491ms | total cached: 49205
# 5: 141 => 6412 in 80.391ms ~ 201.035ms | total cached: 49625
# 6: 2306 => 38477 in 523.623ms ~ 1.207s | total cached: 63102
 -> NoSolution in 4.180s
# search with maxComparisons = 17# 7: 26957 => 143375 in 3.269s ~ 5.392s | total cached: 170685
 -> NoSolution in 7.431s
time '4.180s' + '7.431s' = '11.611s': n = 11, i = 4, (cache_l: 593922, cache_u: 88704, noSol: 0, bruteForce: 59831), cache = 11982 + 75908 = 87890, comparisons: 18
# search with Pair-Optimisation & maxComparisons = 19# 1: 1 => 1 in 70.187µs ~ 55.284µs | total cached: 253477
# 2: 1 => 3 in 74.481µs ~ 473.486µs | total cached: 253477
 -> FoundSolution in 816.364µs
# search with Pair-Optimisation & maxComparisons = 18# 3: 3 => 42 in 682.852µs ~ 2.928ms | total cached: 253477
# 4: 15 => 602 in 8.178ms ~ 29.943ms | total cached: 253477
# 5: 131 => 6270 in 69.131ms ~ 222.914ms | total cached: 253721
# 6: 1903 => 40768 in 455.009ms ~ 1.549s | total cached: 269573
 -> FoundSolution in 3.242s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 2.734s
# search with maxComparisons = 17# 7: 27337 => 154682 in 2.194s ~ 5.015s | total cached: 421850
 -> NoSolution in 5.002s
time '5.977s' + '5.002s' = '10.978s': n = 11, i = 5, (cache_l: 501100, cache_u: 67173, noSol: 0, bruteForce: 59418), cache = 19279 + 128029 = 147308, comparisons: 18

time '0.000ns' + '0.000ns' = '0.000ns': n = 12, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 19279 + 128029 = 147308, comparisons: 11
time '0.000ns' + '0.000ns' = '0.000ns': n = 12, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = 19279 + 128029 = 147308, comparisons: 14
# search with Pair-Optimisation & maxComparisons = 16# 1: 1 => 1 in 158.455µs ~ 53.295µs | total cached: 506020
# 2: 1 => 5 in 450.437µs ~ 418.416µs | total cached: 506020
# 3: 3 => 61 in 3.187ms ~ 7.223ms | total cached: 506020
# 4: 15 => 590 in 36.105ms ~ 17.538ms | total cached: 506020
 -> NoSolution in 197.025ms
# search with maxComparisons = 16# 5: 62 => 4557 in 211.015ms ~ 344.210ms | total cached: 506065
# 6: 275 => 22046 in 1.391s ~ 1.752s | total cached: 506702
 -> NoSolution in 3.668s
time '197.025ms' + '3.668s' = '3.865s': n = 12, i = 2, (cache_l: 150577, cache_u: 14701, noSol: 0, bruteForce: 6523), cache = 19973 + 133858 = 153831, comparisons: 17
# search with Pair-Optimisation & maxComparisons = 19# 1: 1 => 1 in 100.808µs ~ 64.314µs | total cached: 507916
# 2: 1 => 4 in 238.346µs ~ 730.932µs | total cached: 507916
# 3: 3 => 64 in 2.886ms ~ 2.921ms | total cached: 507916
# 4: 14 => 931 in 27.427ms ~ 37.634ms | total cached: 507916
 -> FoundSolution in 175.998ms
# search with Pair-Optimisation & maxComparisons = 18# 5: 140 => 9668 in 304.292ms ~ 327.479ms | total cached: 508036
 -> FoundSolution in 2.549s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 103.564ms
# search with maxComparisons = 17# 6: 1449 => 68737 in 2.911s ~ 2.308s | total cached: 512494
 -> NoSolution in 4.907s
time '2.829s' + '4.907s' = '7.736s': n = 12, i = 3, (cache_l: 489551, cache_u: 45012, noSol: 0, bruteForce: 32187), cache = 22161 + 163857 = 186018, comparisons: 18
# search with Pair-Optimisation & maxComparisons = 22# 1: 1 => 1 in 69.779µs ~ 60.480µs | total cached: 523101
# 2: 1 => 4 in 144.450µs ~ 492.366µs | total cached: 523101
 -> FoundSolution in 4.676ms
# search with Pair-Optimisation & maxComparisons = 21# 3: 3 => 69 in 1.346ms ~ 6.486ms | total cached: 523101
# 4: 15 => 1133 in 24.087ms ~ 50.313ms | total cached: 523101
# 5: 147 => 14424 in 236.918ms ~ 545.379ms | total cached: 523405
# 6: 2942 => 126611 in 3.076s ~ 4.990s | total cached: 542524
 -> FoundSolution in 9.461s
# search with Pair-Optimisation & maxComparisons = 20 -> FoundSolution in 368.403ms
# search with Pair-Optimisation & maxComparisons = 19 -> NoSolution in 15.393s
# search with maxComparisons = 19# 7: 51217 => 708374 in 49.149s ~ 34.363s | total cached: 839423
 -> NoSolution in 103.621s
time '25.227s' + '103.621s' = '128.848s': n = 12, i = 4, (cache_l: 7180549, cache_u: 973187, noSol: 0, bruteForce: 515028), cache = 84359 + 616687 = 701046, comparisons: 20
Error: got 20, but expected 19
```
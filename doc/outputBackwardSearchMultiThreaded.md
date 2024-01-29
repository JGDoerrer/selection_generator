# Program Output (backward search multi threaded)
```
# 1: 1 => 1 in 11.304µs ~ 888.577µs | total cached: 2 (found solution)
time '11.304µs + 888.577µs = 899.881µs': n = 2, i = 0, comparisons: 1

# 1: 1 => 1 in 6.083µs ~ 2.431µs | total cached: 2
# 2: 1 => 1 in 2.046µs ~ 1.004µs | total cached: 3 (found solution)
time '8.129µs + 3.435µs = 11.564µs': n = 3, i = 0, comparisons: 2
# 1: 1 => 1 in 2.730µs ~ 1.949µs | total cached: 3
# 2: 1 => 1 in 2.144µs ~ 1.502µs | total cached: 4
# 3: 1 => 1 in 878.000ns ~ 900.000ns | total cached: 5 (found solution)
time '5.752µs + 4.351µs = 10.103µs': n = 3, i = 1, comparisons: 3

# 1: 1 => 1 in 3.725µs ~ 2.997µs | total cached: 5
# 2: 1 => 2 in 9.746µs ~ 176.632µs | total cached: 5
# 3: 1 => 1 in 3.548µs ~ 1.738µs | total cached: 6 (found solution)
time '17.019µs + 181.367µs = 198.386µs': n = 4, i = 0, comparisons: 3
# 1: 1 => 1 in 6.186µs ~ 4.317µs | total cached: 6
# 2: 1 => 1 in 3.359µs ~ 13.514µs | total cached: 7
# 3: 3 => 3 in 13.718µs ~ 285.541µs | total cached: 9
# 4: 3 => 2 in 23.533µs ~ 54.261µs | total cached: 10 (found solution)
time '46.796µs + 357.633µs = 404.429µs': n = 4, i = 1, comparisons: 4

# 1: 1 => 1 in 12.629µs ~ 5.360µs | total cached: 10
# 2: 1 => 2 in 12.559µs ~ 118.482µs | total cached: 10
# 3: 1 => 2 in 16.462µs ~ 76.582µs | total cached: 10
# 4: 1 => 1 in 6.760µs ~ 1.787µs | total cached: 11 (found solution)
time '48.410µs + 202.211µs = 250.621µs': n = 5, i = 0, comparisons: 4
# 1: 1 => 1 in 5.995µs ~ 6.987µs | total cached: 11
# 2: 1 => 2 in 8.743µs ~ 138.799µs | total cached: 11
# 3: 3 => 4 in 64.383µs ~ 449.603µs | total cached: 12
# 4: 5 => 6 in 24.755µs ~ 373.185µs | total cached: 15
# 5: 4 => 3 in 23.740µs ~ 105.478µs | total cached: 16
# 6: 1 => 1 in 7.999µs ~ 2.990µs | total cached: 17 (found solution)
time '135.615µs + 1.077ms = 1.213ms': n = 5, i = 1, comparisons: 6
# 1: 1 => 1 in 7.026µs ~ 7.225µs | total cached: 17
# 2: 1 => 1 in 5.525µs ~ 15.013µs | total cached: 17
# 3: 3 => 6 in 20.012µs ~ 432.516µs | total cached: 20
# 4: 6 => 7 in 39.677µs ~ 341.536µs | total cached: 25
# 5: 6 => 5 in 40.683µs ~ 292.732µs | total cached: 28
# 6: 3 => 2 in 32.153µs ~ 182.198µs | total cached: 29 (found solution)
time '145.076µs + 1.271ms = 1.416ms': n = 5, i = 2, comparisons: 6

# 1: 1 => 1 in 15.058µs ~ 9.655µs | total cached: 29
# 2: 1 => 3 in 61.140µs ~ 196.766µs | total cached: 29
# 3: 1 => 3 in 41.269µs ~ 108.390µs | total cached: 29
# 4: 1 => 2 in 35.078µs ~ 176.667µs | total cached: 29
# 5: 1 => 1 in 19.075µs ~ 3.187µs | total cached: 30 (found solution)
time '171.620µs + 494.665µs = 666.285µs': n = 6, i = 0, comparisons: 5
# 1: 1 => 1 in 10.761µs ~ 11.209µs | total cached: 30
# 2: 1 => 2 in 31.978µs ~ 257.310µs | total cached: 30
# 3: 3 => 8 in 66.230µs ~ 446.605µs | total cached: 31
# 4: 6 => 13 in 85.422µs ~ 1.270ms | total cached: 34
# 5: 8 => 13 in 66.708µs ~ 669.168µs | total cached: 38
# 6: 5 => 3 in 42.597µs ~ 385.732µs | total cached: 39
# 7: 2 => 2 in 56.182µs ~ 99.522µs | total cached: 40 (found solution)
time '359.878µs + 3.139ms = 3.499ms': n = 6, i = 1, comparisons: 7
# 1: 1 => 1 in 41.672µs ~ 15.404µs | total cached: 40
# 2: 1 => 2 in 15.632µs ~ 381.127µs | total cached: 40
# 3: 3 => 10 in 47.047µs ~ 766.119µs | total cached: 44
# 4: 12 => 19 in 88.619µs ~ 1.230ms | total cached: 54
# 5: 19 => 22 in 67.692µs ~ 955.332µs | total cached: 68
# 6: 18 => 15 in 41.455µs ~ 1.221ms | total cached: 74
# 7: 7 => 6 in 48.166µs ~ 293.719µs | total cached: 77
# 8: 3 => 2 in 15.813µs ~ 242.868µs | total cached: 78 (found solution)
time '366.096µs + 5.106ms = 5.472ms': n = 6, i = 2, comparisons: 8

# 1: 1 => 1 in 64.563µs ~ 15.460µs | total cached: 78
# 2: 1 => 3 in 76.311µs ~ 255.603µs | total cached: 78
# 3: 1 => 4 in 151.494µs ~ 643.267µs | total cached: 78
# 4: 1 => 3 in 125.253µs ~ 379.249µs | total cached: 78
# 5: 1 => 2 in 82.117µs ~ 372.746µs | total cached: 78
# 6: 1 => 1 in 77.051µs ~ 8.379µs | total cached: 79 (found solution)
time '576.789µs + 1.675ms = 2.251ms': n = 7, i = 0, comparisons: 6
# 1: 1 => 1 in 22.124µs ~ 18.369µs | total cached: 79
# 2: 1 => 3 in 40.965µs ~ 382.978µs | total cached: 79
# 3: 3 => 11 in 186.829µs ~ 648.822µs | total cached: 80
# 4: 7 => 24 in 342.369µs ~ 1.923ms | total cached: 82
# 5: 10 => 31 in 328.049µs ~ 1.610ms | total cached: 87
# 6: 11 => 16 in 100.498µs ~ 864.193µs | total cached: 92
# 7: 7 => 5 in 191.352µs ~ 321.588µs | total cached: 93
# 8: 2 => 2 in 36.272µs ~ 424.006µs | total cached: 94 (found solution)
time '1.248ms + 6.193ms = 7.442ms': n = 7, i = 1, comparisons: 8
# 1: 1 => 1 in 18.111µs ~ 19.426µs | total cached: 94
# 2: 1 => 2 in 22.823µs ~ 182.550µs | total cached: 94
# 3: 3 => 15 in 98.389µs ~ 880.709µs | total cached: 95
# 4: 14 => 41 in 353.468µs ~ 1.423ms | total cached: 111
# 5: 40 => 75 in 384.123µs ~ 1.316ms | total cached: 147
# 6: 55 => 75 in 285.413µs ~ 1.386ms | total cached: 177
# 7: 38 => 41 in 172.851µs ~ 1.215ms | total cached: 191
# 8: 17 => 15 in 51.694µs ~ 1.002ms | total cached: 195
# 9: 5 => 4 in 43.618µs ~ 283.787µs | total cached: 196
# 10: 1 => 1 in 8.256µs ~ 33.788µs | total cached: 197 (found solution)
time '1.439ms + 7.742ms = 9.181ms': n = 7, i = 2, comparisons: 10
# 1: 1 => 1 in 23.483µs ~ 17.712µs | total cached: 197
# 2: 1 => 2 in 35.858µs ~ 223.904µs | total cached: 197
# 3: 3 => 11 in 213.095µs ~ 970.860µs | total cached: 197
# 4: 12 => 45 in 394.178µs ~ 1.921ms | total cached: 220
# 5: 43 => 73 in 476.292µs ~ 1.855ms | total cached: 289
# 6: 87 => 86 in 358.423µs ~ 1.448ms | total cached: 356
# 7: 74 => 56 in 137.546µs ~ 908.465µs | total cached: 388
# 8: 35 => 22 in 65.519µs ~ 779.016µs | total cached: 396
# 9: 9 => 6 in 34.824µs ~ 716.264µs | total cached: 399
# 10: 3 => 2 in 11.368µs ~ 174.834µs | total cached: 400 (found solution)
time '1.751ms + 9.016ms = 10.766ms': n = 7, i = 3, comparisons: 10

# 1: 1 => 1 in 67.777µs ~ 12.280µs | total cached: 400
# 2: 1 => 4 in 147.993µs ~ 373.521µs | total cached: 400
# 3: 1 => 5 in 151.771µs ~ 718.451µs | total cached: 400
# 4: 1 => 5 in 164.520µs ~ 523.004µs | total cached: 400
# 5: 1 => 3 in 146.680µs ~ 287.872µs | total cached: 400
# 6: 1 => 2 in 86.070µs ~ 117.022µs | total cached: 400
# 7: 1 => 1 in 41.240µs ~ 3.290µs | total cached: 401 (found solution)
time '806.051µs + 2.035ms = 2.841ms': n = 8, i = 0, comparisons: 7
# 1: 1 => 1 in 23.724µs ~ 19.015µs | total cached: 401
# 2: 1 => 3 in 51.459µs ~ 436.672µs | total cached: 401
# 3: 3 => 14 in 355.868µs ~ 1.640ms | total cached: 401
# 4: 7 => 43 in 754.669µs ~ 1.975ms | total cached: 402
# 5: 11 => 72 in 1.029ms ~ 1.951ms | total cached: 408
# 6: 17 => 51 in 661.761µs ~ 1.082ms | total cached: 414
# 7: 13 => 21 in 446.374µs ~ 1.458ms | total cached: 420
# 8: 8 => 7 in 385.264µs ~ 1.002ms | total cached: 422
# 9: 3 => 2 in 94.191µs ~ 159.241µs | total cached: 423 (found solution)
time '3.802ms + 9.723ms = 13.525ms': n = 8, i = 1, comparisons: 9
# 1: 1 => 1 in 26.173µs ~ 26.174µs | total cached: 423
# 2: 1 => 3 in 39.624µs ~ 320.330µs | total cached: 423
# 3: 3 => 23 in 291.505µs ~ 1.305ms | total cached: 423
# 4: 15 => 82 in 690.173µs ~ 1.868ms | total cached: 432
# 5: 51 => 213 in 1.411ms ~ 3.120ms | total cached: 492
# 6: 123 => 292 in 2.106ms ~ 4.039ms | total cached: 590
# 7: 140 => 216 in 1.145ms ~ 3.805ms | total cached: 656
# 8: 84 => 95 in 1.046ms ~ 2.866ms | total cached: 679
# 9: 28 => 28 in 359.828µs ~ 1.188ms | total cached: 686
# 10: 8 => 6 in 65.566µs ~ 660.021µs | total cached: 688
# 11: 3 => 3 in 120.909µs ~ 385.900µs | total cached: 689 (found solution)
time '7.302ms + 19.585ms = 26.887ms': n = 8, i = 2, comparisons: 11
# 1: 1 => 1 in 38.320µs ~ 27.375µs | total cached: 689
# 2: 1 => 2 in 32.567µs ~ 403.381µs | total cached: 689
# 3: 3 => 21 in 193.501µs ~ 975.946µs | total cached: 689
# 4: 14 => 100 in 618.612µs ~ 3.355ms | total cached: 712
# 5: 86 => 265 in 1.422ms ~ 3.612ms | total cached: 891
# 6: 305 => 467 in 1.978ms ~ 5.396ms | total cached: 1259
# 7: 471 => 444 in 1.930ms ~ 4.037ms | total cached: 1550
# 8: 339 => 259 in 1.124ms ~ 3.559ms | total cached: 1663
# 9: 124 => 90 in 513.070µs ~ 2.205ms | total cached: 1695
# 10: 35 => 25 in 126.176µs ~ 1.039ms | total cached: 1703
# 11: 9 => 7 in 67.271µs ~ 346.774µs | total cached: 1706
# 12: 3 => 2 in 10.369µs ~ 107.564µs | total cached: 1707 (found solution)
time '8.053ms + 25.065ms = 33.117ms': n = 8, i = 3, comparisons: 12

# 1: 1 => 1 in 67.288µs ~ 14.993µs | total cached: 1707
# 2: 1 => 4 in 164.571µs ~ 303.635µs | total cached: 1707
# 3: 1 => 7 in 297.129µs ~ 451.856µs | total cached: 1707
# 4: 1 => 6 in 338.098µs ~ 608.204µs | total cached: 1707
# 5: 1 => 5 in 377.126µs ~ 784.623µs | total cached: 1707
# 6: 1 => 3 in 378.553µs ~ 288.087µs | total cached: 1707
# 7: 1 => 2 in 192.833µs ~ 191.214µs | total cached: 1707
# 8: 1 => 1 in 91.590µs ~ 4.397µs | total cached: 1708 (found solution)
time '1.907ms + 2.647ms = 4.554ms': n = 9, i = 0, comparisons: 8
# 1: 1 => 1 in 37.168µs ~ 23.362µs | total cached: 1708
# 2: 1 => 4 in 103.566µs ~ 341.875µs | total cached: 1708
# 3: 3 => 20 in 618.931µs ~ 945.222µs | total cached: 1708
# 4: 7 => 68 in 1.672ms ~ 1.495ms | total cached: 1709
# 5: 12 => 150 in 3.851ms ~ 3.637ms | total cached: 1713
# 6: 21 => 154 in 3.638ms ~ 3.381ms | total cached: 1722
# 7: 22 => 76 in 2.967ms ~ 2.560ms | total cached: 1729
# 8: 15 => 29 in 1.346ms ~ 1.300ms | total cached: 1736
# 9: 10 => 8 in 349.741µs ~ 433.720µs | total cached: 1738
# 10: 3 => 3 in 160.634µs ~ 309.680µs | total cached: 1739
# 11: 1 => 1 in 10.820µs ~ 18.115µs | total cached: 1740 (found solution)
time '14.756ms + 14.446ms = 29.202ms': n = 9, i = 1, comparisons: 11
# 1: 1 => 1 in 33.139µs ~ 30.497µs | total cached: 1740
# 2: 1 => 3 in 67.478µs ~ 208.115µs | total cached: 1740
# 3: 3 => 29 in 454.729µs ~ 1.053ms | total cached: 1740
# 4: 15 => 147 in 1.747ms ~ 2.412ms | total cached: 1743
# 5: 55 => 529 in 4.786ms ~ 7.185ms | total cached: 1793
# 6: 179 => 997 in 10.575ms ~ 13.772ms | total cached: 2001
# 7: 353 => 1025 in 9.635ms ~ 17.205ms | total cached: 2229
# 8: 314 => 594 in 6.422ms ~ 11.239ms | total cached: 2359
# 9: 159 => 198 in 3.504ms ~ 5.359ms | total cached: 2403
# 10: 53 => 51 in 699.080µs ~ 1.763ms | total cached: 2416
# 11: 16 => 16 in 508.575µs ~ 773.086µs | total cached: 2419
# 12: 4 => 3 in 165.568µs ~ 170.073µs | total cached: 2420 (found solution)
time '38.597ms + 61.171ms = 99.767ms': n = 9, i = 2, comparisons: 12
# 1: 1 => 1 in 42.389µs ~ 33.206µs | total cached: 2420
# 2: 1 => 3 in 53.108µs ~ 202.178µs | total cached: 2420
# 3: 3 => 26 in 332.775µs ~ 1.268ms | total cached: 2420
# 4: 14 => 201 in 2.218ms ~ 4.211ms | total cached: 2436
# 5: 115 => 790 in 5.702ms ~ 13.229ms | total cached: 2712
# 6: 650 => 1963 in 14.703ms ~ 28.744ms | total cached: 3916
# 7: 1776 => 2832 in 21.163ms ~ 37.960ms | total cached: 5631
# 8: 2114 => 2362 in 14.579ms ~ 37.583ms | total cached: 6721
# 9: 1235 => 1083 in 7.501ms ~ 19.654ms | total cached: 7104
# 10: 424 => 342 in 2.599ms ~ 6.387ms | total cached: 7204
# 11: 110 => 94 in 1.052ms ~ 2.343ms | total cached: 7233
# 12: 33 => 22 in 234.510µs ~ 816.423µs | total cached: 7238
# 13: 6 => 5 in 160.091µs ~ 242.631µs | total cached: 7239
# 14: 1 => 1 in 7.534µs ~ 3.626µs | total cached: 7240 (found solution)
time '70.349ms + 152.677ms = 223.026ms': n = 9, i = 3, comparisons: 14
# 1: 1 => 1 in 34.457µs ~ 45.046µs | total cached: 7240
# 2: 1 => 2 in 53.179µs ~ 167.123µs | total cached: 7240
# 3: 3 => 27 in 259.412µs ~ 1.174ms | total cached: 7240
# 4: 14 => 183 in 1.947ms ~ 4.009ms | total cached: 7246
# 5: 94 => 833 in 5.315ms ~ 13.696ms | total cached: 7615
# 6: 686 => 2242 in 13.809ms ~ 35.484ms | total cached: 9708
# 7: 2564 => 3411 in 19.032ms ~ 47.681ms | total cached: 13044
# 8: 3669 => 3153 in 13.905ms ~ 45.673ms | total cached: 15323
# 9: 2404 => 1523 in 6.756ms ~ 28.687ms | total cached: 16144
# 10: 857 => 490 in 2.214ms ~ 9.802ms | total cached: 16354
# 11: 218 => 130 in 596.002µs ~ 3.390ms | total cached: 16413
# 12: 61 => 29 in 181.168µs ~ 1.399ms | total cached: 16423
# 13: 11 => 7 in 92.330µs ~ 372.116µs | total cached: 16426
# 14: 3 => 2 in 12.458µs ~ 104.198µs | total cached: 16427 (found solution)
time '64.208ms + 191.684ms = 255.892ms': n = 9, i = 4, comparisons: 14

# 1: 1 => 1 in 152.374µs ~ 24.504µs | total cached: 16427
# 2: 1 => 5 in 423.830µs ~ 310.806µs | total cached: 16427
# 3: 1 => 10 in 682.879µs ~ 1.485ms | total cached: 16427
# 4: 1 => 9 in 1.102ms ~ 500.695µs | total cached: 16427
# 5: 1 => 7 in 812.590µs ~ 864.985µs | total cached: 16427
# 6: 1 => 5 in 788.946µs ~ 383.755µs | total cached: 16427
# 7: 1 => 3 in 536.089µs ~ 290.091µs | total cached: 16427
# 8: 1 => 2 in 308.662µs ~ 113.823µs | total cached: 16427
# 9: 1 => 1 in 191.877µs ~ 4.527µs | total cached: 16428 (found solution)
time '5.000ms + 3.979ms = 8.978ms': n = 10, i = 0, comparisons: 9
# 1: 1 => 1 in 64.240µs ~ 29.987µs | total cached: 16428
# 2: 1 => 4 in 177.073µs ~ 411.931µs | total cached: 16428
# 3: 3 => 26 in 1.198ms ~ 1.463ms | total cached: 16428
# 4: 7 => 108 in 4.504ms ~ 6.388ms | total cached: 16429
# 5: 13 => 300 in 13.553ms ~ 8.991ms | total cached: 16432
# 6: 24 => 415 in 19.979ms ~ 20.945ms | total cached: 16443
# 7: 33 => 282 in 19.363ms ~ 12.940ms | total cached: 16455
# 8: 27 => 112 in 11.403ms ~ 5.590ms | total cached: 16463
# 9: 19 => 36 in 5.598ms ~ 2.584ms | total cached: 16471
# 10: 11 => 12 in 2.292ms ~ 925.889µs | total cached: 16473
# 11: 3 => 3 in 217.287µs ~ 308.063µs | total cached: 16474
# 12: 2 => 2 in 351.771µs ~ 125.094µs | total cached: 16475 (found solution)
time '78.700ms + 60.702ms = 139.403ms': n = 10, i = 1, comparisons: 12
# 1: 1 => 1 in 50.165µs ~ 42.300µs | total cached: 16475
# 2: 1 => 4 in 124.670µs ~ 369.799µs | total cached: 16475
# 3: 3 => 38 in 770.296µs ~ 1.532ms | total cached: 16475
# 4: 15 => 252 in 4.656ms ~ 4.778ms | total cached: 16477
# 5: 59 => 1177 in 19.706ms ~ 23.467ms | total cached: 16510
# 6: 216 => 3087 in 48.454ms ~ 68.527ms | total cached: 16795
# 7: 647 => 4499 in 74.840ms ~ 124.394ms | total cached: 17393
# 8: 918 => 3640 in 70.052ms ~ 153.997ms | total cached: 17885
# 9: 655 => 1540 in 37.866ms ~ 70.188ms | total cached: 18119
# 10: 289 => 433 in 17.483ms ~ 23.088ms | total cached: 18197
# 11: 94 => 115 in 7.431ms ~ 4.966ms | total cached: 18217
# 12: 24 => 22 in 1.266ms ~ 897.851µs | total cached: 18224
# 13: 8 => 4 in 407.089µs ~ 246.389µs | total cached: 18226
# 14: 2 => 2 in 7.464µs ~ 95.339µs | total cached: 18227 (found solution)
time '283.116ms + 476.588ms = 759.704ms': n = 10, i = 2, comparisons: 14
# 1: 1 => 1 in 32.409µs ~ 37.018µs | total cached: 18227
# 2: 1 => 3 in 68.975µs ~ 275.861µs | total cached: 18227
# 3: 3 => 36 in 583.074µs ~ 1.268ms | total cached: 18227
# 4: 14 => 360 in 4.550ms ~ 7.323ms | total cached: 18237
# 5: 130 => 2063 in 20.788ms ~ 41.699ms | total cached: 18513
# 6: 991 => 7221 in 75.601ms ~ 168.032ms | total cached: 21038
# 7: 4529 => 15488 in 161.796ms ~ 277.293ms | total cached: 28027
# 8: 9310 => 18177 in 197.029ms ~ 376.323ms | total cached: 35476
# 9: 8806 => 11593 in 127.397ms ~ 243.671ms | total cached: 39305
# 10: 4291 => 4748 in 61.870ms ~ 227.060ms | total cached: 40530
# 11: 1343 => 1311 in 24.246ms ~ 96.160ms | total cached: 40849
# 12: 355 => 283 in 6.872ms ~ 7.427ms | total cached: 40911
# 13: 69 => 54 in 3.024ms ~ 3.108ms | total cached: 40924
# 14: 14 => 11 in 346.147µs ~ 800.358µs | total cached: 40928
# 15: 5 => 4 in 632.378µs ~ 277.909µs | total cached: 40929 (found solution)
time '684.837ms + 1.451s = 2.136s': n = 10, i = 3, comparisons: 15
# 1: 1 => 1 in 45.885µs ~ 50.486µs | total cached: 40929
# 2: 1 => 3 in 88.670µs ~ 316.544µs | total cached: 40929
# 3: 3 => 40 in 463.814µs ~ 2.344ms | total cached: 40929
# 4: 15 => 379 in 3.928ms ~ 11.042ms | total cached: 40933
# 5: 126 => 2506 in 20.289ms ~ 65.982ms | total cached: 41386
# 6: 1497 => 10169 in 84.431ms ~ 286.180ms | total cached: 48079
# 7: 10592 => 24726 in 228.224ms ~ 691.847ms | total cached: 70915
# 8: 28245 => 34314 in 313.043ms ~ 678.773ms | total cached: 98491
# 9: 31075 => 25543 in 198.624ms ~ 495.043ms | total cached: 114019
# 10: 16762 => 11710 in 88.343ms ~ 257.491ms | total cached: 119113
# 11: 5410 => 3475 in 32.204ms ~ 98.593ms | total cached: 120369
# 12: 1345 => 772 in 9.481ms ~ 20.311ms | total cached: 120605
# 13: 252 => 159 in 2.160ms ~ 5.301ms | total cached: 120654
# 14: 53 => 36 in 596.763µs ~ 1.875ms | total cached: 120665
# 15: 13 => 8 in 612.856µs ~ 719.110µs | total cached: 120668
# 16: 3 => 2 in 11.614µs ~ 165.811µs | total cached: 120669 (found solution)
time '982.548ms + 2.616s = 3.599s': n = 10, i = 4, comparisons: 16

# 1: 1 => 1 in 262.722µs ~ 28.500µs | total cached: 120669
# 2: 1 => 5 in 914.791µs ~ 1.759ms | total cached: 120669
# 3: 1 => 12 in 1.877ms ~ 1.046ms | total cached: 120669
# 4: 1 => 13 in 2.217ms ~ 3.829ms | total cached: 120669
# 5: 1 => 10 in 3.672ms ~ 753.505µs | total cached: 120669
# 6: 1 => 7 in 1.879ms ~ 1.747ms | total cached: 120669
# 7: 1 => 5 in 2.066ms ~ 870.356µs | total cached: 120669
# 8: 1 => 3 in 1.515ms ~ 568.734µs | total cached: 120669
# 9: 1 => 2 in 845.961µs ~ 174.269µs | total cached: 120669
# 10: 1 => 1 in 555.108µs ~ 6.944µs | total cached: 120670 (found solution)
time '15.803ms + 10.783ms = 26.586ms': n = 11, i = 0, comparisons: 10
# 1: 1 => 1 in 129.033µs ~ 37.175µs | total cached: 120670
# 2: 1 => 5 in 409.625µs ~ 608.716µs | total cached: 120670
# 3: 3 => 33 in 2.793ms ~ 3.774ms | total cached: 120670
# 4: 7 => 149 in 14.645ms ~ 10.396ms | total cached: 120670
# 5: 13 => 559 in 63.312ms ~ 38.916ms | total cached: 120672
# 6: 26 => 1043 in 130.989ms ~ 75.113ms | total cached: 120682
# 7: 43 => 950 in 149.796ms ~ 97.889ms | total cached: 120701
# 8: 46 => 448 in 106.802ms ~ 48.655ms | total cached: 120715
# 9: 33 => 135 in 44.766ms ~ 13.218ms | total cached: 120725
# 10: 22 => 50 in 20.758ms ~ 4.754ms | total cached: 120734
# 11: 12 => 13 in 6.858ms ~ 2.117ms | total cached: 120736
# 12: 4 => 5 in 1.873ms ~ 794.494µs | total cached: 120737
# 13: 2 => 2 in 702.764µs ~ 464.688µs | total cached: 120738 (found solution)
time '543.834ms + 296.737ms = 840.572ms': n = 11, i = 1, comparisons: 13
# 1: 1 => 1 in 150.964µs ~ 44.163µs | total cached: 120738
# 2: 1 => 4 in 198.856µs ~ 667.830µs | total cached: 120738
# 3: 3 => 48 in 1.639ms ~ 4.034ms | total cached: 120738
# 4: 15 => 388 in 11.084ms ~ 14.365ms | total cached: 120738
# 5: 61 => 2389 in 57.327ms ~ 53.499ms | total cached: 120766
# 6: 251 => 8665 in 235.075ms ~ 263.819ms | total cached: 121075
# 7: 971 => 18206 in 615.485ms ~ 759.516ms | total cached: 122226
# 8: 2081 => 20205 in 949.957ms ~ 1.376s | total cached: 123735
# 9: 2167 => 11521 in 653.136ms ~ 921.359ms | total cached: 124698
# 10: 1254 => 3984 in 303.958ms ~ 422.258ms | total cached: 125085
# 11: 480 => 985 in 153.062ms ~ 299.207ms | total cached: 125200
# 12: 138 => 184 in 32.038ms ~ 13.466ms | total cached: 125234
# 13: 42 => 37 in 7.662ms ~ 4.683ms | total cached: 125244
# 14: 12 => 9 in 1.369ms ~ 1.519ms | total cached: 125246
# 15: 3 => 3 in 1.312ms ~ 154.710µs | total cached: 125247 (found solution)
time '3.023s + 4.135s = 7.158s': n = 11, i = 2, comparisons: 15
# 1: 1 => 1 in 57.061µs ~ 43.056µs | total cached: 125247
# 2: 1 => 4 in 127.200µs ~ 720.535µs | total cached: 125247
# 3: 3 => 50 in 1.129ms ~ 3.617ms | total cached: 125247
# 4: 14 => 594 in 12.963ms ~ 26.472ms | total cached: 125251
# 5: 139 => 4684 in 78.935ms ~ 133.209ms | total cached: 125466
# 6: 1265 => 23591 in 450.701ms ~ 579.279ms | total cached: 129236
# 7: 8689 => 74458 in 1.556s ~ 1.991s | total cached: 149289
# 8: 29958 => 124081 in 2.773s ~ 4.577s | total cached: 185741
# 9: 45644 => 111686 in 2.650s ~ 5.059s | total cached: 215177
# 10: 33918 => 60961 in 1.515s ~ 4.954s | total cached: 227997
# 11: 14219 => 20391 in 571.360ms ~ 3.169s | total cached: 231729
# 12: 4099 => 4420 in 266.800ms ~ 601.650ms | total cached: 232515
# 13: 859 => 767 in 73.585ms ~ 57.537ms | total cached: 232674
# 14: 175 => 149 in 11.244ms ~ 10.767ms | total cached: 232712
# 15: 44 => 35 in 5.830ms ~ 1.515ms | total cached: 232720
# 16: 9 => 5 in 1.563ms ~ 659.399µs | total cached: 232722
# 17: 2 => 2 in 11.410µs ~ 237.279µs | total cached: 232723 (found solution)
time '9.969s + 21.167s = 31.136s': n = 11, i = 3, comparisons: 17
# 1: 1 => 1 in 50.403µs ~ 44.455µs | total cached: 232723
# 2: 1 => 3 in 79.211µs ~ 940.147µs | total cached: 232723
# 3: 3 => 53 in 1.326ms ~ 4.987ms | total cached: 232723
# 4: 15 => 674 in 14.905ms ~ 26.044ms | total cached: 232723
# 5: 141 => 6412 in 69.168ms ~ 217.505ms | total cached: 233142
# 6: 2306 => 38477 in 523.033ms ~ 1.112s | total cached: 246597
# 7: 27030 => 144056 in 2.699s ~ 4.236s | total cached: 343828
# 8: 132681 => 296296 in 6.980s ~ 8.597s | total cached: 563844
# 9: 258380 => 330671 in 7.356s ~ 11.150s | total cached: 772705
# 10: 229332 => 213300 in 3.739s ~ 10.849s | total cached: 873527
# 11: 107396 => 84651 in 1.351s ~ 6.565s | total cached: 903480
# 12: 31597 => 23071 in 394.522ms ~ 1.694s | total cached: 909956
# 13: 6784 => 4610 in 141.351ms ~ 1.104s | total cached: 911195
# 14: 1302 => 829 in 27.751ms ~ 102.985ms | total cached: 911402
# 15: 222 => 161 in 7.697ms ~ 7.574ms | total cached: 911449
# 16: 51 => 34 in 1.902ms ~ 2.187ms | total cached: 911457
# 17: 9 => 6 in 1.564ms ~ 497.679µs | total cached: 911458
# 18: 1 => 1 in 17.784µs ~ 9.698µs | total cached: 911459 (found solution)
time '23.309s + 45.668s = 68.978s': n = 11, i = 4, comparisons: 18
# 1: 1 => 1 in 61.766µs ~ 103.573µs | total cached: 911459
# 2: 1 => 3 in 94.427µs ~ 465.411µs | total cached: 911459
# 3: 3 => 42 in 732.259µs ~ 2.421ms | total cached: 911459
# 4: 15 => 602 in 6.740ms ~ 23.677ms | total cached: 911459
# 5: 131 => 6270 in 59.717ms ~ 213.612ms | total cached: 911703
# 6: 1903 => 40768 in 424.438ms ~ 1.231s | total cached: 927545
# 7: 27357 => 154968 in 2.044s ~ 5.036s | total cached: 1079614
# 8: 180741 => 336347 in 5.728s ~ 10.253s | total cached: 1473920
# 9: 424945 => 402465 in 8.456s ~ 12.097s | total cached: 1872417
# 10: 414724 => 272256 in 4.449s ~ 11.514s | total cached: 2070451
# 11: 203314 => 113149 in 1.477s ~ 6.808s | total cached: 2129563
# 12: 60437 => 31661 in 371.638ms ~ 2.776s | total cached: 2142622
# 13: 13303 => 6440 in 67.181ms ~ 2.156s | total cached: 2145077
# 14: 2508 => 1115 in 12.837ms ~ 87.832ms | total cached: 2145487
# 15: 422 => 213 in 3.271ms ~ 9.843ms | total cached: 2145567
# 16: 83 => 38 in 842.867µs ~ 2.267ms | total cached: 2145581
# 17: 15 => 8 in 751.663µs ~ 597.749µs | total cached: 2145584
# 18: 3 => 2 in 12.926µs ~ 104.168µs | total cached: 2145585 (found solution)
time '23.103s + 52.212s = 75.315s': n = 11, i = 5, comparisons: 18
```
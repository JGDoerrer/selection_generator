# Installation
Nauty Manual: [https://pallini.di.uniroma1.it/nug28.pdf]()
```sh
cd $project_folder$
wget https://users.cecs.anu.edu.au/~bdm/nauty/nauty2_8_8.tar.gz
tar xvzf nauty2_8_8.tar.gz
rm nauty2_8_8.tar.gz
cd nauty2_8_8
./configure
make
cd ../src
g++ -O3 nautyTest.cpp ../nauty2_8_8/nauty.a
./a.out
```

# Annahmen
- keine Duplikate in der Eingabe

# Poset-Datenstruktur
|     | 0 | 1 | 2 | 3 | ... |
| -   | - | - | - | - |  -  |
| 0   | `f` |   | `t` |   |     |
| 1   |   | `f` |   |   |     |
| 2   |   |   | `f` |   |     |
| 3   |   |   |   | `f` |     |
| ... |   |   |   |   |  `f`  |

- Einträge mit `f` sind immer false
- wenn Eintrag `(0, 2) == true` gilt list[0] < list[2]
- => dann kann nicht `(2, 0) == true` gelten


# Algorithmus
- starte mit 1 sicherem Vergleich (0, 1)
- berechne mittels brute-force und Paar-Bildung am Anfang Schranke `n` (relativ schnell)
- berechne mittels brute-force ohne Paar-Bildung, ob mit weniger als `n` möglich (also `...`, `n - 2`, `n - 1`)
- wenn nein: untere Schranke ist `n`, sonst: Fall tritt nicht ein

# Program Output
Rust:
```
...
found solution for n = 10, i = 4, comparisons = 16
cache entries: 85055
time since start: 0d 0h 0m 9.324393s
...
```

C++:
Mit Nauty:
```
...
time '0.000s': n = 9, i = 0, calls = 0, hits = 0, cache = (258 + 1201 = 1459), comparisons: 8
time '0.012s': n = 9, i = 1, calls = 58, hits = 1591, cache = (267 + 1238 = 1505), comparisons: 11
time '0.059s': n = 9, i = 2, calls = 510, hits = 13646, cache = (303 + 1558 = 1861), comparisons: 12
time '0.767s': n = 9, i = 3, calls = 9640, hits = 245092, cache = (1341 + 5979 = 7320), comparisons: 14
time '1.032s': n = 9, i = 4, calls = 16472, hits = 414086, cache = (2486 + 14407 = 16893), comparisons: 14

time '0.000s': n = 10, i = 0, calls = 0, hits = 0, cache = (2486 + 14407 = 16893), comparisons: 9
time '0.007s': n = 10, i = 1, calls = 9, hits = 334, cache = (2488 + 14412 = 16900), comparisons: 12
time '0.345s': n = 10, i = 2, calls = 1306, hits = 43870, cache = (2636 + 15258 = 17894), comparisons: 14
time '2.218s': n = 10, i = 3, calls = 13870, hits = 468299, cache = (3208 + 25134 = 28342), comparisons: 15
time '10.399s': n = 10, i = 4, calls = 91850, hits = 2959530, cache = (7436 + 79396 = 86832), comparisons: 16

time '0.000s': n = 11, i = 0, calls = 0, hits = 0, cache = (7436 + 79396 = 86832), comparisons: 10
time '0.041s': n = 11, i = 1, calls = 34, hits = 1366, cache = (7440 + 79421 = 86861), comparisons: 13
time '1.517s': n = 11, i = 2, calls = 2126, hits = 89572, cache = (7570 + 80853 = 88423), comparisons: 15
time '41.588s': n = 11, i = 3, calls = 134177, hits = 5344767, cache = (15175 + 152130 = 167305), comparisons: 17
time '211.426s': n = 11, i = 4, calls = 1030173, hits = 40542832, cache = (58834 + 647374 = 706208), comparisons: 18
```

Ohne nauty:
```
...
time '20.869s': n = 9, i = 3, calls = 1189770, hits = 29240161, entries = 642345, comparisons: 14
time '48.006s': n = 9, i = 4, calls = 2474876, hits = 58721753, entries = 1566708, comparisons: 14
```
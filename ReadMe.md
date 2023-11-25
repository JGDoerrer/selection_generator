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
found solution for n = 9, i = 4, comparisons = 14
time since start: 0d 0h 0m 18.30282s
cache_entries = 17149
...
```

With pair-optimisation, Nauty:
```
time '0.000s': n = 8, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 7
time '0.052s': n = 8, i = 1, calls = 2166, hits = 25071, entries = 3858, comparisons: 9
time '0.580s': n = 8, i = 2, calls = 31289, hits = 254611, entries = 10423, comparisons: 11
time '0.625s': n = 8, i = 3, calls = 50713, hits = 328686, entries = 8487, comparisons: 12

time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 8
time '1.817s': n = 9, i = 1, calls = 63522, hits = 989771, entries = 52380, comparisons: 11
time '7.409s': n = 9, i = 2, calls = 345845, hits = 3577103, entries = 99155, comparisons: 12
time '33.365s': n = 9, i = 3, calls = 2794813, hits = 14884652, entries = 134791, comparisons: 14
time '26.390s': n = 9, i = 4, calls = 2459714, hits = 11151854, entries = 128926, comparisons: 14
```

Without pair-optimisation, Nauty:
```
time '0.000s': n = 8, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 7
time '0.157s': n = 8, i = 1, calls = 6674, hits = 81141, entries = 6643, comparisons: 9
time '1.523s': n = 8, i = 2, calls = 108545, hits = 653420, entries = 15479, comparisons: 11
time '2.967s': n = 8, i = 3, calls = 385152, hits = 1283968, entries = 19463, comparisons: 12

time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 8
time '3.068s': n = 9, i = 1, calls = 105602, hits = 1648032, entries = 67358, comparisons: 11
time '9.717s': n = 9, i = 2, calls = 564781, hits = 4988731, entries = 108041, comparisons: 12
```
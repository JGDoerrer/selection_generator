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

Ohne nauty:
```
time '20.869s': n = 9, i = 3, calls = 1189770, hits = 29240161, entries = 642345, comparisons: 14
time '48.006s': n = 9, i = 4, calls = 2474876, hits = 58721753, entries = 1566708, comparisons: 14
```

C++:
Mit Nauty:
```
time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 1383, comparisons: 8
time '0.011s': n = 9, i = 1, calls = 82, hits = 1510, entries = 1239, comparisons: 11
time '0.051s': n = 9, i = 2, calls = 6373, hits = 8987, entries = 1463, comparisons: 12
time '1.387s': n = 9, i = 3, calls = 490422, hits = 275684, entries = 5901, comparisons: 14
time '2.228s': n = 9, i = 4, calls = 903977, hits = 563969, entries = 11912, comparisons: 14

time '0.000s': n = 10, i = 0, calls = 0, hits = 0, entries = 11912, comparisons: 9
time '0.007s': n = 10, i = 1, calls = 36, hits = 426, entries = 11077, comparisons: 12
time '0.586s': n = 10, i = 2, calls = 133718, hits = 71212, entries = 12220, comparisons: 14
time '4.058s': n = 10, i = 3, calls = 1138077, hits = 639919, entries = 21542, comparisons: 15
time '42.193s': n = 10, i = 4, calls = 18686475, hits = 8403543, entries = 73907, comparisons: 16

time '0.000s': n = 11, i = 0, calls = 0, hits = 0, entries = 73907, comparisons: 10
time '0.040s': n = 11, i = 1, calls = 74, hits = 1467, entries = 68871, comparisons: 13
time '1.595s': n = 11, i = 2, calls = 163649, hits = 95085, entries = 70243, comparisons: 15
time '78.254s': n = 11, i = 3, calls = 26293192, hits = 9238534, entries = 127267, comparisons: 17
```

Ohne Korrektheitsprüfung (die eigentlich nötig ist):
```
time '401.387s': n = 11, i = 4, calls = 158932254, hits = 69168443, entries = 391589, comparisons: 18
time '1181.766s': n = 11, i = 5, calls = 560583279, hits = 245805272, entries = 964590, comparisons: 18

time '0.000s': n = 12, i = 0, calls = 0, hits = 0, entries = 964590, comparisons: 11
time '0.039s': n = 12, i = 1, calls = 701, hits = 443, entries = 964596, comparisons: 14
time '1.306s': n = 12, i = 2, calls = 151801, hits = 57464, entries = 965069, comparisons: 17
time '69.142s': n = 12, i = 3, calls = 12920604, hits = 6102451, entries = 1007559, comparisons: 18
```
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
time since start: 0d 0h 0m 9.493829s
cache_entries = 15811
...
```

With pair-optimisation, Nauty:
```
time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 664, comparisons: 8
time '0.007s': n = 9, i = 1, calls = 2179, hits = 921, entries = 686, comparisons: 11
time '0.046s': n = 9, i = 2, calls = 7676, hits = 9095, entries = 882, comparisons: 12
time '0.891s': n = 9, i = 3, calls = 289797, hits = 200544, entries = 4396, comparisons: 14
time '1.421s': n = 9, i = 4, calls = 539208, hits = 375196, entries = 9164, comparisons: 14

time '0.000s': n = 10, i = 0, calls = 0, hits = 0, entries = 9164, comparisons: 9
time '0.005s': n = 10, i = 1, calls = 1319, hits = 285, entries = 9168, comparisons: 12
time '0.121s': n = 10, i = 2, calls = 28786, hits = 16187, entries = 9427, comparisons: 14
time '1.419s': n = 10, i = 3, calls = 367950, hits = 242425, entries = 12406, comparisons: 15
time '13.687s': n = 10, i = 4, calls = 4991376, hits = 2916645, entries = 37135, comparisons: 16

time '0.000s': n = 11, i = 0, calls = 0, hits = 0, entries = 37135, comparisons: 10
time '0.029s': n = 11, i = 1, calls = 905, hits = 1537, entries = 37164, comparisons: 13
time '1.119s': n = 11, i = 2, calls = 194185, hits = 90883, entries = 38188, comparisons: 15
time '35.316s': n = 11, i = 3, calls = 9559597, hits = 4631519, entries = 75261, comparisons: 17
time '401.387s': n = 11, i = 4, calls = 158932254, hits = 69168443, entries = 391589, comparisons: 18
time '1181.766s': n = 11, i = 5, calls = 560583279, hits = 245805272, entries = 964590, comparisons: 18

time '0.000s': n = 12, i = 0, calls = 0, hits = 0, entries = 964590, comparisons: 11
time '0.039s': n = 12, i = 1, calls = 701, hits = 443, entries = 964596, comparisons: 14
time '1.306s': n = 12, i = 2, calls = 151801, hits = 57464, entries = 965069, comparisons: 17
time '69.142s': n = 12, i = 3, calls = 12920604, hits = 6102451, entries = 1007559, comparisons: 18
```

Without pair-optimisation, Nauty:
```
time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 2147, comparisons: 8
time '0.168s': n = 9, i = 1, calls = 98726, hits = 16145, entries = 2189, comparisons: 11
time '0.309s': n = 9, i = 2, calls = 115053, hits = 44272, entries = 2668, comparisons: 12
time '3.814s': n = 9, i = 3, calls = 1805009, hits = 670576, entries = 7515, comparisons: 14
time '11.035s': n = 9, i = 4, calls = 6147501, hits = 2466426, entries = 18560, comparisons: 14

time '0.000s': n = 10, i = 0, calls = 0, hits = 0, entries = 18560, comparisons: 9
time '0.078s': n = 10, i = 1, calls = 29490, hits = 5418, entries = 18597, comparisons: 12
time '2.789s': n = 10, i = 2, calls = 1131476, hits = 271045, entries = 20071, comparisons: 14
time '26.592s': n = 10, i = 3, calls = 11811882, hits = 3713443, entries = 38126, comparisons: 15
time '245.871s': n = 10, i = 4, calls = 132577505, hits = 44515853, entries = 114513, comparisons: 16

time '0.000s': n = 11, i = 0, calls = 0, hits = 0, entries = 114513, comparisons: 10
time '0.300s': n = 11, i = 1, calls = 83001, hits = 13564, entries = 114572, comparisons: 13
time '15.251s': n = 11, i = 2, calls = 5354596, hits = 1009827, entries = 117320, comparisons: 15
time '383.775s': n = 11, i = 3, calls = 166932146, hits = 42084813, entries = 205042, comparisons: 17
```
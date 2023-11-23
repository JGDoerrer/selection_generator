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
With pair-optimisation:
```
time '0.000s': n = 6, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 5
time '0.000s': n = 6, i = 1, calls = 115, hits = 546, entries = 16, comparisons: 7
time '0.001s': n = 6, i = 2, calls = 608, hits = 3189, entries = 260, comparisons: 8

time '0.000s': n = 7, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 6
time '0.014s': n = 7, i = 1, calls = 2598, hits = 23147, entries = 3488, comparisons: 8
time '0.086s': n = 7, i = 2, calls = 18023, hits = 171447, entries = 1347, comparisons: 10
time '0.090s': n = 7, i = 3, calls = 20208, hits = 182585, entries = 4612, comparisons: 10

time '0.000s': n = 8, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 7
time '0.068s': n = 8, i = 1, calls = 6668, hits = 83405, entries = 15120, comparisons: 9
time '1.215s': n = 8, i = 2, calls = 142857, hits = 1905142, entries = 70118, comparisons: 11
time '2.574s': n = 8, i = 3, calls = 337000, hits = 4432577, entries = 79261, comparisons: 12

time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 8
time '9.877s': n = 9, i = 1, calls = 522635, hits = 10671008, entries = 130112, comparisons: 11
time '41.368s': n = 9, i = 2, calls = 2617621, hits = 49400409, entries = 1621682, comparisons: 12
```

Without pair-optimisation:
```
time '0.000s': n = 6, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 5
time '0.007s': n = 6, i = 1, calls = 2395, hits = 17131, entries = 1215, comparisons: 7
time '0.015s': n = 6, i = 2, calls = 5141, hits = 35740, entries = 1550, comparisons: 8

time '0.000s': n = 7, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 6
time '0.146s': n = 7, i = 1, calls = 25356, hits = 286523, entries = 13833, comparisons: 8
time '0.400s': n = 7, i = 2, calls = 78771, hits = 835263, entries = 14997, comparisons: 10
time '0.468s': n = 7, i = 3, calls = 90375, hits = 962335, entries = 17444, comparisons: 10

time '0.000s': n = 8, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 7
time '3.372s': n = 8, i = 1, calls = 307396, hits = 5053822, entries = 189389, comparisons: 9
time '10.951s': n = 8, i = 2, calls = 1119934, hits = 17205686, entries = 248841, comparisons: 11
time '16.524s': n = 8, i = 3, calls = 1838967, hits = 27231485, entries = 266394, comparisons: 12

time '0.000s': n = 9, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 8
```
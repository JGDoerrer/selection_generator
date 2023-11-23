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
```
time '0.000s': n = 1, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 0

time '0.000s': n = 2, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 1

time '0.000s': n = 3, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 2
time '0.000s': n = 3, i = 1, calls = 8, hits = 16, entries = 7, comparisons: 3

time '0.000s': n = 4, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 3
time '0.000s': n = 4, i = 1, calls = 27, hits = 87, entries = 29, comparisons: 4

time '0.000s': n = 5, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 4
time '0.000s': n = 5, i = 1, calls = 307, hits = 1432, entries = 332, comparisons: 6
time '0.000s': n = 5, i = 2, calls = 280, hits = 1181, entries = 299, comparisons: 6

time '0.000s': n = 6, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 5
time '0.017s': n = 6, i = 1, calls = 2401, hits = 15130, entries = 3557, comparisons: 7
time '0.050s': n = 6, i = 2, calls = 5853, hits = 34783, entries = 6120, comparisons: 8

time '0.000s': n = 7, i = 0, calls = 0, hits = 0, entries = 0, comparisons: 6
time '1.601s': n = 7, i = 1, calls = 23888, hits = 213598, entries = 49890, comparisons: 8
time '94.543s': n = 7, i = 2, calls = 218841, hits = 1825195, entries = 244388, comparisons: 10
```
# Installation
Nauty Manual: [https://pallini.di.uniroma1.it/nug28.pdf]()
```sh
cd $project_folder$
wget https://users.cecs.anu.edu.au/~bdm/nauty/nauty2_8_8.tar.gz
tar xvzf nauty2_8_8.tar.gz
rm nauty2_8_8.tar.gz
cd nauty2_8_8
./configure --enable-tls
make
cd ../src
make nauty
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

# Program Results
C++:
```
...
time '0.000s': n = 9, i = 0, calls = 0, hits = 0, cache = (266 + 1213 = 1479), comparisons: 8
time '0.010s': n = 9, i = 1, calls = 53, hits = 1452, cache = (276 + 1250 = 1526), comparisons: 11
time '0.049s': n = 9, i = 2, calls = 437, hits = 11727, cache = (302 + 1527 = 1829), comparisons: 12
time '0.594s': n = 9, i = 3, calls = 7834, hits = 199667, cache = (1003 + 5532 = 6535), comparisons: 14
time '1.028s': n = 9, i = 4, calls = 16519, hits = 414119, cache = (2168 + 13947 = 16115), comparisons: 14

time '0.000s': n = 10, i = 0, calls = 0, hits = 0, cache = (2168 + 13947 = 16115), comparisons: 9
time '0.006s': n = 10, i = 1, calls = 9, hits = 332, cache = (2170 + 13952 = 16122), comparisons: 12
time '0.301s': n = 10, i = 2, calls = 1168, hits = 39113, cache = (2293 + 14742 = 17035), comparisons: 14
time '1.881s': n = 10, i = 3, calls = 12088, hits = 405880, cache = (2827 + 23219 = 26046), comparisons: 15
time '7.705s': n = 10, i = 4, calls = 70922, hits = 2283108, cache = (6089 + 68186 = 74275), comparisons: 16

time '0.000s': n = 11, i = 0, calls = 0, hits = 0, cache = (6089 + 68186 = 74275), comparisons: 10
time '0.040s': n = 11, i = 1, calls = 35, hits = 1351, cache = (6094 + 68211 = 74305), comparisons: 13
time '1.242s': n = 11, i = 2, calls = 1801, hits = 75489, cache = (6204 + 69451 = 75655), comparisons: 15
time '29.333s': n = 11, i = 3, calls = 100514, hits = 3995027, cache = (10876 + 125602 = 136478), comparisons: 17
time '213.733s': n = 11, i = 4, calls = 1059549, hits = 41419058, cache = (56968 + 632129 = 689097), comparisons: 18
time '262.824s': n = 11, i = 5, calls = 1647805, hits = 63416786, cache = (108502 + 1488856 = 1597358), comparisons: 18

time '0.000s': n = 12, i = 0, calls = 0, hits = 0, cache = (108502 + 1488856 = 1597358), comparisons: 11
time '0.089s': n = 12, i = 1, calls = 21, hits = 1010, cache = (108505 + 1488867 = 1597372), comparisons: 14
time '16.276s': n = 12, i = 2, calls = 9117, hits = 461013, cache = (109234 + 1494450 = 1603684), comparisons: 17
time '118.978s': n = 12, i = 3, calls = 172686, hits = 8580873, cache = (113111 + 1619369 = 1732480), comparisons: 18
```
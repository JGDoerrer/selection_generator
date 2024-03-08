# Lower Bounds for Selection 13, 14, 15 Elements
## short overview (Stand: 8.3.24)
| n  | k | forward search | backward search | bidirection search |
| -  | - | -         | -         | -         |
| 10 | 0 | 0s        | 910.834µs | 0s        |
| 10 | 1 | 0s        | 18.267ms  | 0s        |
| 10 | 2 | 0s        | 149.528ms | 0s        |
| 10 | 3 | 244.385ms | 949.857ms | 136.426ms |
| 10 | 4 | 1.759s    | 3.071s    | 909.580ms |
| -  | - | -         | -         | -         |
| 11 | 0 | 0s        | 1.875ms   | 0s        |
| 11 | 1 | 0s        | 70.571ms  | 0s        |
| 11 | 2 | 0s        | 1.135s    | 0s        |
| 11 | 3 | 3.368s    | 11.039s   | 1.716s    |
| 11 | 4 | 19.037s   | 41.733s   | 8.386s    |
| 11 | 5 | 15.507s   | 57.641s   | 7.585s    |
| -  | - | -         | -         | -         |
| 12 | 0 | 0s        | 3.960ms   | 0s        |
| 12 | 1 | 0s        | 324.382ms | 0s        |
| 12 | 2 | 2.453s    | 10.417s   | 2.066s    |
| 12 | 3 | 11.058s   | 168.505s  | 5.464s    |
| 12 | 4 | 258.417s  | ?         | 81.013s   |
| 12 | 5 | 473.053s  | ?         | 189.009s  |

## Nekton-Server
im Uni VPN: `ssh [username]@nekton.informatik.uni-stuttgart.de`

## Tricks
### Implementierte Tricks nach Nützlichkeit absteigen sortiert
Im folgenden sei `maxN` die Größe des größten Posets, `n` die Größe eines spezifischen Posets und das `i`-kleinste Element gesucht. Es wurden folgende Optimierungen angewendet:
- Führe die iterative deepening Suche zunächst mit dem Trick der Paar-Bildung aus. Wenn eine Anzahl an Vergleichen gefunden ist (sei diese `k`), führe eine iterative deepening Suche OHNE den Trick aus mit einer Schranke von maximal `k - 1` Vergleichen aus. Wenn diese keine Lösung finde, ist `k` die Lösung, anderenfalls ist der Paare-Trick widerlegt und es muss normal gesucht werden (siehe code)
- benutze iterative deepening mit einer Schranke, die bei `lowerBound` (aus der Theorie) startet und bei `upperBound` (aus der Theorie) endet
- benutze iterative deepening parallel auf- sowie absteigend
- wenn das `i`-kleinste Element gesucht ist, ist die Anzahl an Vergleichen identisch zu dem `n - i`-kleinsten Element
- wenn zu einem Poset ein Vergleich hinzugefügt wird, überprüfe, ob aus den bereits gespeicherten Vergleichen und dem hinzugefügten neue Vergleiche durch transitive Beziehungen folgen
- speichere Lösungen zu Posets in caches:
  - `cache_lower`: wenn Poset `p` in `cache_lower` drin, kann `p` **nicht gelöst** werden, wenn **HÖCHSTENS** noch `cache_lower[p]` Vergleiche in der `search`-Funktion verfügbar sind.
  - `cache_upper`: wenn Poset `p` in `cache_upper` drin, kann `p` **gelöst** werden, wenn **MINDESTENS** noch `cache_upper[p]` Vergleiche in der `search`-Funktion verfügbar sind.
- benutze statt zwei großen Caches viele kleine -> verbessert mutex-lock-Wartezeiten bei Nebenläufigkeit und Caches werden für sich kleiner
- normalisiere ein Posets wie folgt:
  - wenn das `i`-kleinste Element gesucht ist und bereits bekannt, dass ein Element größer als z.B. alle anderen ist, kann dieses Element entfernt werden => dadurch ändert sich potentiell die Größe vom Poset
  - überführe das Poset anschließend mittels `nauty` in eine kanonische Normalform
- wenn ein Poset (als Graph dargestellt) `k` Zusammenhangskomponenten hat, braucht man mindestens noch `k - 1` Vergleiche zum lösen
- randomisiere die Reihenfolge, in der `i`, `j` in der Brute-Force-Suche ausgewählt werden (alternativ: sortiere diese nach Ihren Erfolgschancen; funktioniert ok, in der Praxis jedoch langsam)
- wenn `i == 0`, werden immer `n - 1` Vergleiche benötigt

### weitere Überlegungen / Arbeitspunkte
- Parallelisiere die Suche durch den Suchbaum mittels Threadpool -> Problem: Suchbaum nicht symmetrisch, Verbesserung nur marginal
- Mache eine Rückwärtssuche für `q`-Vergleiche (z.B. `q` = 4) und befülle die beiden caches inital. In der Suche können sich die letzten `q` Ebenen im Suchbaum gespart werden, da entweder das Poset im cache vorhanden ist oder wenn nicht unmöglich zu lösen in `q`-Vergleichen -> Problem: nur führ sehr kleine `q` möglich
- speichere zu den aktuell in der Tiefensuche verwendeten Posets extra Informationen, anstatt neu zu berechnen (jedoch nicht in Caches)

## Poset-Datenstruktur
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
- Definiere Poset `p` ist `closed` gdw. jede transitive '1' gesetzt ist (d.h. es ex. kein a, b, c mit `is_less(a, b) && is_less(b, c) && !is_less(a, c)`)

## Literatur
- [C-implementation Oksanen](https://www.cs.hut.fi/~cessu/selection/)
- [The Art of Computer Programming (TAOCP)](https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3), [online pdf](https://doc.lagout.org/science/0_Computer%20Science/2_Algorithms/The%20Art%20of%20Computer%20Programming%20%28vol.%203_%20Sorting%20and%20Searching%29%20%282nd%20ed.%29%20%5BKnuth%201998-05-04%5D.pdf)
- [Lower Bounds for Sorting](https://arxiv.org/pdf/2206.05597.pdf), [reorderGraphCanonically](https://github.com/CodeCrafter47/sortinglowerbounds/blob/92865960ba465e4b6e068b400da82ff3f12af803/src/expandedPoset.cpp#L65)
- [Nauty Manual](https://pallini.di.uniroma1.it/nug28.pdf)
- [Anzahl der linearen Erweiterungen](https://arxiv.org/pdf/1108.0866.pdf)
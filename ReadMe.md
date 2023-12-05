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

# Algorithmus
Zur Übersichtlichkeit ohne Caching und Multithreading:
```cpp
template <size_t maxN>
SearchResult search(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
  SearchResult result = NoSolution;
  if (poset.canDetermineNSmallest()) {
    result = FoundSolution;
  } else if (0 != remainingComparisons) {
    for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
      for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
        if (!poset.is(i, j) && !poset.is(j, i)) {
          result = search(createPosetWithComparison(poset, i, j), remainingComparisons - 1);
          if (result == FoundSolution) {
            result = search(createPosetWithComparison(poset, j, i), remainingComparisons - 1);
          }
        }
      }
    }
  }
  return result;
}

template <size_t maxN>
std::optional<int> startSearchIterative(const int n, const int nthSmallest) {
  // Der Fall `0 == nthSmallest` ist bereits bekannt
  if (0 == nthSmallest || n <= 2) {
    return n - 1;
  }

  // `maxDepth` gibt die maximale Suchtiefe an. Diese wird, solange kein Ergebnis gefunden wurde, iterativ erhöht
  for (int maxDepth = n - 1; maxDepth < n * n; ++maxDepth) {
    int comparisonsDone = 0;
    Poset<maxN> poset{n, nthSmallest};  // erstelle ein leeres Poset
    for (int k = 0; k < n - 1 && comparisonsDone < maxDepth; k += 2) {
      poset.addComparison(k, k + 1);  // bilde inital Paar und vergleiche diese
      ++comparisonsDone;  // reduziere unsere maximale Suchtiefe, da bereits ein Vergleich durchgeführt wurde
    }
    // Suche, ob durch hinzufügen von maximal `maxDepth` Vergleichen, das Poset gelöst werden kann
    if (FoundSolution == searchIterative(poset, maxDepth - comparisonsDone)) {
      // Bis jetzt ist bekannt, dass mit dem "Paare-Trick" das i-kleinste Element in dem Poset in `maxDepth`-Schritten
      // eindeutig gelöst werden kann. Da der Trick mit den Paaren nicht bewiesen ist, führe anschließend noch eine
      // normale Suche mit Tiefe `maxDepth - 1` durch. Wenn diese in `NoSolution` resultiert, ist die Lösung gefunden
      // (anderenfalls hätten wir den "Paare-Trick" widerlegt)
      Poset<maxN> poset{n, nthSmallest};
      // da irgendwelche 2 Elemente am Anfang verglichen werden, wähle o.B.d.A `0` und `1`
      int comparisonsDone = 1;
      poset.addComparison(0, 1);

      if (NoSolution == searchIterative(poset, maxDepth - comparisonsDone - 1)) {
        return maxDepth;
      }

      std::cout << "Error: \"Paare-Trick\" widerlegt" << std::endl;
      return {};
    }
  }
}
```

# Tricks
## Implementierte Tricks nach Nützlichkeit absteigen sortiert
Im folgenden sei `maxN` die Größe des größten Posets, `n` die Größe eines spezifischen Posets und das `i`-kleinste Element gesucht. Es wurden folgende Optimierungen angewendet:
- Führe die iterative deepening Suche zunächst mit dem Trick der Paar-Bildung aus. Wenn eine Anzahl an Vergleichen gefunden ist (sei diese `k`), führe eine iterative deepening Suche OHNE den Trick aus mit einer Schranke von maximal `k - 1` Vergleichen aus. Wenn diese keine Lösung finde, ist `k` die Lösung, anderenfalls ist der Paare-Trick widerlegt und es muss normal gesucht werden (siehe code)
- benutze iterative deepening mit einer Schranke, die bei `n - 1` startet (da jedes Poset mit `n` Elementen und beliebigem `i` min. `n - 1` Vergleiche braucht) und iterativ nach oben geht (siehe code)
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

## weitere Überlegungen / Arbeitspunkte
- Parallelisiere die Suche durch den Suchbaum mittels Threadpool -> Problem: Suchbaum nicht symmetrisch, Verbesserung nur marginal
- Mache eine Rückwärtssuche für `q`-Vergleiche (z.B. `q` = 4) und befülle die beiden caches inital. In der Suche können sich die letzten `q` Ebenen im Suchbaum gespart werden, da entweder das Poset im cache vorhanden ist oder wenn nicht unmöglich zu lösen in `q`-Vergleichen -> Problem: nur führ sehr kleine `q` möglich
- speichere zu den aktuell in der Tiefensuche verwendeten Posets extra Informationen, anstatt neu zu berechnen (jedoch nicht in Caches)

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

# Program Output
C++:
```
...
time '0.000s + 0.000s = 0.000s': n = 9, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1044 + 219 = 1263), comparisons: 8
time '0.000s + 0.000s = 0.000s': n = 9, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1044 + 219 = 1263), comparisons: 11
time '0.000s + 0.000s = 0.000s': n = 9, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1044 + 219 = 1263), comparisons: 12
# search with Pair-Optimisation & maxComparisons = 13 -> NoSolution in 0.269s
# search with maxComparisons = 13 -> NoSolution in 0.165s
time '0.269s + 0.165s = 0.434s': n = 9, i = 3, (cache_l: 123818, cache_u: 25754, noSol: 8, bruteForce: 6002), cache = (4864 + 870 = 5734), comparisons: 14
# search with Pair-Optimisation & maxComparisons = 15 -> FoundSolution in 0.000s
# search with Pair-Optimisation & maxComparisons = 14 -> FoundSolution in 0.346s
# search with Pair-Optimisation & maxComparisons = 13 -> NoSolution in 0.082s
# search with maxComparisons = 13 -> NoSolution in 0.116s
time '0.428s + 0.116s = 0.545s': n = 9, i = 4, (cache_l: 190295, cache_u: 44617, noSol: 7, bruteForce: 9268), cache = (11406 + 1691 = 13097), comparisons: 14

time '0.000s + 0.000s = 0.000s': n = 10, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (11406 + 1691 = 13097), comparisons: 9
time '0.000s + 0.000s = 0.000s': n = 10, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (11406 + 1691 = 13097), comparisons: 12
time '0.000s + 0.000s = 0.000s': n = 10, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (11406 + 1691 = 13097), comparisons: 14
# search with Pair-Optimisation & maxComparisons = 14 -> NoSolution in 0.238s
# search with maxComparisons = 14 -> NoSolution in 1.354s
time '0.238s + 1.354s = 1.593s': n = 10, i = 3, (cache_l: 308065, cache_u: 43176, noSol: 5, bruteForce: 10553), cache = (19450 + 2155 = 21605), comparisons: 15
# search with Pair-Optimisation & maxComparisons = 16 -> FoundSolution in 0.181s
# search with Pair-Optimisation & maxComparisons = 15 -> NoSolution in 1.696s
# search with maxComparisons = 15 -> NoSolution in 4.924s
time '1.878s + 4.924s = 6.803s': n = 10, i = 4, (cache_l: 1875883, cache_u: 279524, noSol: 41, bruteForce: 68037), cache = (65602 + 5595 = 71197), comparisons: 16

time '0.000s + 0.000s = 0.000s': n = 11, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (65602 + 5595 = 71197), comparisons: 10
time '0.000s + 0.000s = 0.000s': n = 11, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (65602 + 5595 = 71197), comparisons: 13
time '0.000s + 0.000s = 0.000s': n = 11, i = 2, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (65602 + 5595 = 71197), comparisons: 15
# search with Pair-Optimisation & maxComparisons = 18 -> FoundSolution in 1.199s
# search with Pair-Optimisation & maxComparisons = 17 -> FoundSolution in 2.656s
# search with Pair-Optimisation & maxComparisons = 16 -> NoSolution in 7.111s
# search with maxComparisons = 16 -> NoSolution in 10.749s
time '10.967s + 10.749s = 21.716s': n = 11, i = 3, (cache_l: 2868695, cache_u: 378266, noSol: 28, bruteForce: 82523), cache = (119490 + 10255 = 129745), comparisons: 17
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 67.619s
# search with maxComparisons = 17 -> NoSolution in 34.669s
time '67.619s + 34.669s = 102.289s': n = 11, i = 4, (cache_l: 19653011, cache_u: 2339754, noSol: 308, bruteForce: 565760), cache = (479281 + 31544 = 510825), comparisons: 18
# search with Pair-Optimisation & maxComparisons = 19 -> FoundSolution in 2.139s
# search with Pair-Optimisation & maxComparisons = 18 -> FoundSolution in 56.272s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 33.809s
# search with maxComparisons = 17 -> NoSolution in 36.656s
time '92.221s + 36.656s = 128.878s': n = 11, i = 5, (cache_l: 30085265, cache_u: 3915540, noSol: 127, bruteForce: 883085), cache = (1094857 + 61112 = 1155969), comparisons: 18

time '0.000s + 0.000s = 0.000s': n = 12, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1094857 + 61112 = 1155969), comparisons: 11
time '0.000s + 0.000s = 0.000s': n = 12, i = 1, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1094857 + 61112 = 1155969), comparisons: 14
# search with Pair-Optimisation & maxComparisons = 16 -> NoSolution in 0.681s
# search with maxComparisons = 16 -> NoSolution in 13.902s
time '0.681s + 13.902s = 14.584s': n = 12, i = 2, (cache_l: 405364, cache_u: 49858, noSol: 12, bruteForce: 9099), cache = (1101291 + 61807 = 1163098), comparisons: 17
# search with Pair-Optimisation & maxComparisons = 19 -> FoundSolution in 0.003s
# search with Pair-Optimisation & maxComparisons = 18 -> FoundSolution in 14.092s
# search with Pair-Optimisation & maxComparisons = 17 -> NoSolution in 2.664s
# search with maxComparisons = 17 -> NoSolution in 67.648s
time '16.760s + 67.648s = 84.409s': n = 12, i = 3, (cache_l: 6276184, cache_u: 579414, noSol: 56, bruteForce: 137414), cache = (1204968 + 65377 = 1270345), comparisons: 18
# search with Pair-Optimisation & maxComparisons = 22 -> FoundSolution in 0.065s
# search with Pair-Optimisation & maxComparisons = 21 -> FoundSolution in 15.201s
# search with Pair-Optimisation & maxComparisons = 20 -> FoundSolution in 3.602s
# search with Pair-Optimisation & maxComparisons = 19 -> NoSolution in 482.740s
# search with maxComparisons = 19 -> FoundSolution in 935.443s
# search with Pair-Optimisation & maxComparisons = 18 -> NoSolution in 0.000s
# search with maxComparisons = 18 -> NoSolution in 0.568s
time '501.610s + 936.012s = 1437.622s': n = 12, i = 4, (cache_l: 192046780, cache_u: 18939768, noSol: 1458, bruteForce: 4485921), cache = (3927728 + 201904 = 4129632), comparisons: 19
# search with Pair-Optimisation & maxComparisons = 20 -> FoundSolution in 53.172s
# search with Pair-Optimisation & maxComparisons = 19 -> NoSolution in 611.049s
# search with maxComparisons = 19
```

# (theoretische) Schranken
```
n = 1, t = 0: 0

n = 2, t = 0: 1

n = 3, t = 0: 2
n = 3, t = 1: 3

n = 4, t = 0: 3
n = 4, t = 1: 4

n = 5, t = 0: 4
n = 5, t = 1: 6
n = 5, t = 2: 6

n = 6, t = 0: 5
n = 6, t = 1: 7
n = 6, t = 2: 8

n = 7, t = 0: 6
n = 7, t = 1: 8
n = 7, t = 2: 9 - 10
n = 7, t = 3: 9 - 12

n = 8, t = 0: 7
n = 8, t = 1: 9
n = 8, t = 2: 11
n = 8, t = 3: 11 - 13

n = 9, t = 0: 8
n = 9, t = 1: 11
n = 9, t = 2: 12
n = 9, t = 3: 13 - 14
n = 9, t = 4: 12 - 16

n = 10, t = 0: 9
n = 10, t = 1: 12
n = 10, t = 2: 14
n = 10, t = 3: 14 - 15
n = 10, t = 4: 14 - 17

n = 11, t = 0: 10
n = 11, t = 1: 13
n = 11, t = 2: 15
n = 11, t = 3: 16 - 19
n = 11, t = 4: 16 - 18
n = 11, t = 5: 15 - 20

n = 12, t = 0: 11
n = 12, t = 1: 14
n = 12, t = 2: 16 - 17
n = 12, t = 3: 17 - 20
n = 12, t = 4: 18 - 23
n = 12, t = 5: 17 - 21

n = 13, t = 0: 12
n = 13, t = 1: 15
n = 13, t = 2: 17 - 18
n = 13, t = 3: 19 - 21
n = 13, t = 4: 19 - 24
n = 13, t = 5: 19 - 26
n = 13, t = 6: 18 - 24

n = 14, t = 0: 13
n = 14, t = 1: 16
n = 14, t = 2: 19
n = 14, t = 3: 20 - 22
n = 14, t = 4: 21 - 25
n = 14, t = 5: 21 - 28
n = 14, t = 6: 20 - 27

n = 15, t = 0: 14
n = 15, t = 1: 17
n = 15, t = 2: 20
n = 15, t = 3: 22 - 23
n = 15, t = 4: 22 - 26
n = 15, t = 5: 23 - 29
n = 15, t = 6: 22 - 30
n = 15, t = 7: 21 - 35
```
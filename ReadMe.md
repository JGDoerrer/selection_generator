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
- randomisiere die Reihenfolge, in der `i`, `j` in der Suche ausgewählt werden -> oder sortiere diese nach Ihren Erfolgschancen (Code Josua)
- wenn `i == 0`, werden immer `n - 1` Vergleiche benötigt

## weitere Überlegungen / Arbeitspunkte
- Parallelisiere die Suche durch den Suchbaum mittels Threadpool -> Problem: Suchbaum nicht symmetrisch, Verbesserung nur marginal
- Mache eine Rückwärtssuche für `q`-Vergleiche (z.B. `q` = 4) und befülle die beiden caches inital. In der Suche können sich die letzten `q` Ebenen im Suchbaum gespart werden, da entweder das Poset im cache vorhanden ist oder wenn nicht unmöglich zu lösen in `q`-Vergleichen -> Problem: nur führ sehr kleine `q` möglich
- speichere zu den aktuell in der Tiefensuche verwendeten Posets extra Informationen, anstatt neu zu berechnen (jedoch nicht in Caches)

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
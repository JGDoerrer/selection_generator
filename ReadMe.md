# Lower Bounds for Selection 13, 14, 15 Elements
## short overview
| n | k | forward search (Bottom to Top) | forward search (Top to Bottom) | backward search (naive) |
| -  | - | - | - | - |
| 10 | 0 | 0.000s | 0.000s | 588.653s |
| 10 | 1 | 0.000s | 0.000s | ~600s |
| 10 | 2 | 0.000s | 0.000s | ~600s |
| 10 | 3 | 0.521s | 0.535s | ~600s |
| 10 | 4 | 5.432s | 5.028s | ~600s |
| ... | ... | ... | ... | ... |
| 11 | 4 | 76.371s | 78.881s | ? |
| 11 | 5 | 119.288s | 114.623s | ? |

## interne Links
- [Github Repo](https://github.com/JGDoerrer/selection_generator/)
- [Theoretische Schranken](./doc/theoreticalBounds.md)
- [Program Output (forward search from Bottom to Top)](./doc/outputForwardSearchBottomTop.md)
- [Program Output (forward search from Top to Bottom)](./doc/outputForwardSearchTopBottom.md)
- [Program Output (backward search)](./doc/outputBackwardSearch.md)

## Installation
- run `sh setup.sh` in the project folder once
- `make -B forwardSearch`
- `make -B forwardSearch opt=debug`
- `make -B backwardSearch`
- `make -B backwardSearch opt=debug`

## Nekton-Server
im Uni VPN: `ssh [username]@nekton.informatik.uni-stuttgart.de`

## Annahmen
- keine Duplikate in der Eingabe

## Algorithmus
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
std::optional<int> startSearch(const int n, const int nthSmallest) {
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
    if (FoundSolution == search(poset, maxDepth - comparisonsDone)) {
      // Bis jetzt ist bekannt, dass mit dem "Paare-Trick" das i-kleinste Element in dem Poset in `maxDepth`-Schritten
      // eindeutig gelöst werden kann. Da der Trick mit den Paaren nicht bewiesen ist, führe anschließend noch eine
      // normale Suche mit Tiefe `maxDepth - 1` durch. Wenn diese in `NoSolution` resultiert, ist die Lösung gefunden
      // (anderenfalls hätten wir den "Paare-Trick" widerlegt)
      Poset<maxN> poset{n, nthSmallest};
      // da irgendwelche 2 Elemente am Anfang verglichen werden, wähle o.B.d.A `0` und `1`
      int comparisonsDone = 1;
      poset.addComparison(0, 1);

      if (NoSolution == search(poset, maxDepth - comparisonsDone - 1)) {
        return maxDepth;
      }

      std::cout << "Error: \"Paare-Trick\" widerlegt" << std::endl;
      return {};
    }
  }
}
```

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
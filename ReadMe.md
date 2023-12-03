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

# Program Results
C++:
```
...
time '0.000s + 0.000s = 0.000s': n = 9, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (863 + 162 = 1025), comparisons: 8
time '0.003s + 0.003s = 0.006s': n = 9, i = 1, (cache_l: 777, cache_u: 113, noSol: 1, bruteForce: 35), cache = (874 + 166 = 1040), comparisons: 11
time '0.031s + 0.024s = 0.056s': n = 9, i = 2, (cache_l: 13593, cache_u: 1680, noSol: 17, bruteForce: 604), cache = (1166 + 196 = 1362), comparisons: 12
time '0.406s + 0.070s = 0.476s': n = 9, i = 3, (cache_l: 165951, cache_u: 28630, noSol: 74, bruteForce: 7847), cache = (4803 + 738 = 5541), comparisons: 14
time '0.803s + 0.063s = 0.866s': n = 9, i = 4, (cache_l: 343522, cache_u: 64198, noSol: 219, bruteForce: 16350), cache = (12655 + 1833 = 14488), comparisons: 14

time '0.000s + 0.000s = 0.000s': n = 10, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (12655 + 1833 = 14488), comparisons: 9
time '0.002s + 0.008s = 0.010s': n = 10, i = 1, (cache_l: 670, cache_u: 24, noSol: 1, bruteForce: 20), cache = (12663 + 1835 = 14498), comparisons: 12
time '0.017s + 0.204s = 0.222s': n = 10, i = 2, (cache_l: 26060, cache_u: 1744, noSol: 31, bruteForce: 824), cache = (13012 + 1843 = 14855), comparisons: 14
time '0.622s + 0.544s = 1.167s': n = 10, i = 3, (cache_l: 272107, cache_u: 29321, noSol: 177, bruteForce: 9243), cache = (17699 + 2082 = 19781), comparisons: 15
time '2.600s + 1.597s = 4.198s': n = 10, i = 4, (cache_l: 1329330, cache_u: 162412, noSol: 839, bruteForce: 47152), cache = (43196 + 3317 = 46513), comparisons: 16

time '0.000s + 0.000s = 0.000s': n = 11, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (43196 + 3317 = 46513), comparisons: 10
time '0.015s + 0.023s = 0.039s': n = 11, i = 1, (cache_l: 1194, cache_u: 68, noSol: 1, bruteForce: 34), cache = (43206 + 3323 = 46529), comparisons: 13
time '0.476s + 0.762s = 1.238s': n = 11, i = 2, (cache_l: 85330, cache_u: 7773, noSol: 40, bruteForce: 2410), cache = (44297 + 3434 = 47731), comparisons: 15
time '17.432s + 3.886s = 21.319s': n = 11, i = 3, (cache_l: 3140524, cache_u: 348143, noSol: 557, bruteForce: 90450), cache = (89516 + 6734 = 96250), comparisons: 17
time '184.454s + 9.229s = 193.683s': n = 11, i = 4, (cache_l: 37361556, cache_u: 4620047, noSol: 3975, bruteForce: 1085444), cache = (590603 + 51481 = 642084), comparisons: 18
time '223.366s + 7.408s = 230.775s': n = 11, i = 5, (cache_l: 55443680, cache_u: 6538462, noSol: 8155, bruteForce: 1614247), cache = (1414082 + 99829 = 1513911), comparisons: 18

time '0.000s + 0.000s = 0.000s': n = 12, i = 0, (cache_l: 0, cache_u: 0, noSol: 0, bruteForce: 0), cache = (1414082 + 99829 = 1513911), comparisons: 11
time '0.032s + 0.093s = 0.125s': n = 12, i = 1, (cache_l: 1561, cache_u: 24, noSol: 1, bruteForce: 32), cache = (1414094 + 99832 = 1513926), comparisons: 14
time '1.221s + 8.163s = 9.384s': n = 12, i = 2, (cache_l: 219521, cache_u: 19037, noSol: 64, bruteForce: 4771), cache = (1416170 + 99979 = 1516149), comparisons: 17
time '48.769s + 29.038s = 77.807s': n = 12, i = 3, (cache_l: 6034354, cache_u: 444381, noSol: 1191, bruteForce: 134227), cache = (1496994 + 102029 = 1599023), comparisons: 18
try: upper comparisonBound = 20 ~20min?
...
```
ACHTUNG: mit n = 12, i = 4 ist der "Paare-Trick" widerlegt

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
# Zadanie projektowe nr 2

## Temat

Badanie efektywności algorytmów grafowych w zależności od rozmiaru instancji oraz sposobu reprezentacji grafu w pamięci komputera.

## Zakres projektu

Należy zaimplementować i zmierzyć czas działania algorytmów grafowych rozwiązujących następujące problemy:

### a. Minimalne drzewo rozpinające (MST)

- Algorytm Prima
- Algorytm Kruskala

### b. Najkrótsza ścieżka w grafie

- Algorytm Dijkstry
- Algorytm Forda-Bellmana

### c. Maksymalny przepływ

- Algorytm Forda-Fulkersona _(tylko na ocenę 5 i 5.5)_

### Reprezentacje grafu:

- macierzowa (macierz sąsiedztwa)
- listowa (lista sąsiadów)

---

## Założenia

- Dynamiczna alokacja struktur danych
- Przepustowość krawędzi jako liczba całkowita
- Pomiar czasu dla każdej implementacji w zależności od:

  - liczby wierzchołków (7 różnych wartości)
  - gęstości: 20%, 60%, 99%
  - 50 losowych instancji na zestaw, wyniki uśrednione

- Generowanie grafu:

  - najpierw drzewo rozpinające
  - następnie pozostałe krawędzie

- Sprawdzenie poprawności struktury i operacji
- Pomiar czasu: `QueryPerformanceCounter` lub `std::chrono::high_resolution_clock`
- Język: kompilowany do kodu natywnego (np. C/C++), inne za zgodą prowadzącego
- Konsolowa wersja programu wystarczająca
- Zakaz używania bibliotek takich jak STL, Boost itp. (chyba że przewidziano wyjątek)
- Jeden program zawierający całą implementację
- Komentowany kod źródłowy

---

## Sprawdzenie poprawności

1. **Wczytanie z pliku tekstowego**:

   - Format:

     - Pierwsza linia: liczba krawędzi, liczba wierzchołków
     - Dodatkowe dane w zależności od algorytmu:

       - Dla najkrótszej ścieżki: wierzchołek początkowy
       - Dla przepływu: wierzchołek startowy i końcowy

     - Kolejne linie: `wierzchołek_początkowy wierzchołek_końcowy waga`

   - Wierzchołki numerowane od zera
   - MST – krawędzie nieskierowane
   - Pozostałe – skierowane

2. **Losowe generowanie grafu**

   - Parametry: liczba wierzchołków, gęstość
   - Nowy graf nadpisuje poprzedni

3. **Wyświetlanie grafu**

   - W formie listowej i macierzowej

4. **Uruchamianie algorytmów**

   - Dla obu reprezentacji
   - Wyniki na ekranie

---

## Menu programu

Po wybraniu problemu:

1. Wczytaj z pliku (wyświetlenie grafu)
2. Wygeneruj graf losowo (wyświetlenie grafu)
3. Wyświetl graf
4. Uruchom Algorytm 1 (macierzowo i listowo)
5. Uruchom Algorytm 2 (macierzowo i listowo)

### Wyświetlanie wyników:

- **MST**: lista krawędzi z wagami + suma wag
- **Najkrótsza ścieżka**: koszt i ścieżka od źródła do każdego wierzchołka

---

## Sprawozdanie

1. **Wstęp** – złożoność obliczeniowa problemów

2. **Plan eksperymentu**

3. **Opis generowania grafu**

4. **Wyniki**:

   - Tabele i wykresy osobno dla każdego problemu
   - Dla MST i najkrótszej ścieżki:

     - **Typ 1**: wykresy dla każdej reprezentacji (2 algorytmy × 3 gęstości)
     - **Typ 2**: wykresy dla każdej gęstości (2 algorytmy × 2 reprezentacje)

   - Analogicznie dla maksymalnego przepływu
   - Oś Y: czas, oś X: liczba wierzchołków

5. **Wnioski**

   - Efektywność struktur
   - Różnice między teorią a praktyką

6. **Załączniki**

   - Kod źródłowy (kompletna wersja i skompilowana)
   - Sprawozdanie papierowe

---

## Uwagi

1. Jeden problem należy oddać wcześniej
2. Naiwna implementacja:

   - Kolejki priorytetowej (tablica) lub zbiorów rozłącznych (tablica kolorów) obniża ocenę

3. Osobna implementacja algorytmów dla każdej reprezentacji (nie wspólny interfejs)
4. Nie upraszczać E jako $V^2$ w złożoności
5. Graf spójny:

   - Nieskierowany: zacząć od drzewa rozpinającego
   - Skierowany: zapewnić połączenie między wszystkimi wierzchołkami (np. pętla)

6. Gęstość 99%:

   - Generować graf pełny, a następnie usuwać 1% krawędzi

---

## Ocena projektu

| Ocena   | Wymagania                                               |
| ------- | ------------------------------------------------------- |
| **3.0** | Dijkstra + Prim/Kruskal (STL dopuszczalny)              |
| **3.5** | Dijkstra + Prim + Kruskal (STL dopuszczalny)            |
| **4.0** | Po dwa algorytmy z każdego problemu (STL dopuszczalny)  |
| **4.4** | Po dwa algorytmy z każdego problemu (bez STL)           |
| **5.0** | Wersja obiektowa (bez STL) + Ford-Fulkerson (DFS)       |
| **5.5** | Wersja obiektowa (bez STL) + Ford-Fulkerson (DFS + BFS) |

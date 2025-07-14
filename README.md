# Algorytmy i Złożoność Obliczeniowa

- Autor: Mateusz Głuchowski

# [Zadanie projektowe nr 1](https://github.com/Ite-2022-pwr/sem4-aizo-proj-mg/tree/master/projekt-1)

Należy wybrać i zaimplementować (samodzielnie) określone algorytmu sortowania, a następnie przeprowadzić analizę ich efektywności. Zakładamy, że po sortowaniu elementy powinny być uporządkowane rosnąco. Wskazane jest użycie szblonów (templates), aby łatwo można było wykorzystać zaimplementowane algorytmy do sortowania tablic zawierających elementy różnych typów (int, float).

## Należy przyjąć następujące założenia:

- Podstawowy element - 4 sortowanych tablic jest 4 bajtowa liczba całkowita (`int`). Dla porównania należy rozpatrzeć również inny typ danych (`float`).

- Wszystkie sortowane tablice powinny być alokowane dynamicznie (zgodnie z badanym rozmiarem tablicy).

- Należy przeprowadzić **wstępną weryfikację porpawności sortowania** zaimplementowanych algorytmów. Dla małych tablic weryfikacja może być przeprowadzona wizualnie. Dla większych tablic może być przydatna procedura pomocnicza sprawdzająca poprawność uporządkowania elementów w tablicy.

- Należy zmierzyć czasy sortowania tablic w funkcji ich rozmiaru. Ponieważ pojedynczy pomiar może być obarczony znacznym błędem oraz otrzymane wyniki mogą zależeć także od rozkładu danych, to pomiary dla konkretnego rozmiaru tablicy należy wykonać wielokrotnie (np. 100 razy - za każdym razem generując nowy zestaw danych), a wyniki uśrednić. W sprawozdaniu podajemy tylko wartości uśrednionie. Badane rozmiary tablic należy dobrać eksperymentalnie w zależności od wydajności sprzętu (badania należy przeprowadzić dla 7 reprezentatywnych rozmiarów tablic np. 10000, 20000, 50000, 100000). Wielkość tablic dobrać tak, aby pomiar czasu sortowania był na poziomie $ms$ i więcej, a nie na $ns$ czy $us$. Pomiar powinien uwzględnić wyłącznie czas sortowania tablicy (nie powinien uwzględniać np. czasu generacji danych).

- Czas sortowania może zależeć od początkowego rozkładu elementów, należy rozpatrzyć przypadki szczególne (i umieścić je w sprawozdaniu): tablica całkowicie losowa tablica posortowana rosnąco, tablica posortowana malejąco, tablica posortowana częściowo (33% i 66% początkowych elementów już posortowanych)

- dodatkową funkcją programu musi być możliwość sprawdzenia poprawności zaimplementowanych
  algorytmów sortowania (wczytanie danych z pliku i wyświetlenie tablicy przed i po sortowaniu)

## Sprawdzenie poprawności algorytmów sortowania obejmuje

- utworzenie dynamicznej tablicy na podstawie zapisanych w pliku tekstowym (każda liczba w osobnym wierszu). Pierwsza liczba określa rozmiar tablicy, pozostałe są kolejnymi elementami tablicy.

- Wyświetlenie na ekranie tablicy przed sortowaniem.

- Wyświetlenie na ekranie tablicy po sortowaniu.

Należy stworzyć menu, które umożliwi następujące operacje:

- Wczytanie tablicy z pliku o zadanej nazwie
- Wyenerowanie tablicy o zadanym rozmiarze zawierające losowe wartości (program ma zapytać o rozmiar tablicy)
- Wyświetlenie ostatnio utworzonej tablicy na ekranie (wygenerowanej lub wczytanej)
- Uruchomienie wybranego algorytmu na ostatnio utworzonej tablicy (do uruchomienia algorytmu używamy kopii utworzonej tablicy) i od razu wyświetlanie tablicy posortowanej
- Wyświetlenie posortowanej tablicy na ekranie.

- Menu można rozszerzyć o własne opcje. W przypadku badania wpływu typu danych na czas sortowania można stworzyć menu dwupoziomowe, gdzie na pierwszym poziomie wybieramy typ danych ( `float` lub `int`) a drugi poziom zawiera menu przedstawione powyżej. Powrót z menu niższego poziomu na wyższy nie jest wymagany.

## Ocena projektu (we wszystkich wersjach bada się zawsze wpływ rozmiaru tablicy oraz wstępnego ułożenia danych w tablicy na czas wykonania algorytmu):

- `3.0` – sortowanie przez wstawianie (insertion sort), przez scalanie (merge sort) i bąbelkowe (buble sort) – dodatkowo badamy wpływ typu danych int i float
- `4.0` – sortowanie przez wstawianie zwykłe i binarne, przez kopcowanie (heap sort) i szybkie - program w wersji obiektowej, dodatkowo badamy wpływ typu danych np. int i float
- `5.0` – sortowanie przez wstawianie, przez kopcowanie, Shella i szybkie - program w wersji obiektowej,
  dodatkowo badamy wpływ typu danych int i float, wpływ wyboru odstępów dla algorytmu Shella
  (2 różne wzory tworzące algorytmy o różnych złożonościach) oraz sposób wyboru pivota (skrajny
  lewy, skrajny prawy, środkowy oraz losowy) dla algorytmu szybkiego.

# [Zadanie projektowe nr 2](https://github.com/Ite-2022-pwr/sem4-aizo-proj-mg/tree/master/projekt-2)

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

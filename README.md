# AiZO - Projekt 1

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
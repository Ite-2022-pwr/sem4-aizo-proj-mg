use std::cmp;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::algorithms::{heap_sort, insert_sort, quick_sort, shell_sort};
use crate::handlers::files_handler;
// use crate::utils::benchmark;
use crate::utils::generator;

use owo_colors::OwoColorize;

#[derive(Clone)]
pub struct Menu {
    datai: Vec<i64>,
    dataf: Vec<f64>,
    datatype: i64, // -1 = None, 0 = f64, 1 = i64
    benchmarking: bool,
}
impl Menu {
    pub fn new(datai: Vec<i64>, dataf: Vec<f64>, datatype: i64, benchmarking: bool) -> Menu {
        Menu {
            datai,
            dataf,
            datatype,
            benchmarking,
        }
    }

    pub fn run(&mut self) {
        let mut user_choice: u8;
        loop {
            // Pierwszy poziom menu
            println!("Menu:\n 1. Wybór typu danych.\n 2. Wczytanie pliku z danymi.\n 3. Wyswietlanie wczytanych danych z pliku.\n 4. Wybor algorytmu sortowania.\n 5. Generowanie danych.\n 6. Zamknij program.");
            self.pretty(String::from("~"));
            user_choice = self.read_user_choice_u8();

            match user_choice {
                1 => {
                    println!("Typy danych:\n 0. f64\n 1. i64");
                    self.pretty("wybor-typu-danych".to_string());
                    self.datatype = self.read_user_choice_i64();
                    if self.datatype != 0 && self.datatype != 1 {
                        println!("{}", "[-] Wrong type. Setting to i64.".red());
                        self.datatype = 1;
                    }
                }

                2 => {
                    // Wybor pliku z danymi
                    files_handler::list_files();
                    self.pretty("wybor-pliku".to_string());
                    let chosen_file = self.read_user_choice_str();
                    let filepath = Path::new("./data/").join(chosen_file);

                    // Wczytanie danych z pliku
                    if files_handler::verify_data_type(&filepath) {
                        // f64
                        self.dataf = files_handler::read_dataf(&filepath);
                        self.datatype = 0;
                    } else {
                        // i64
                        self.datai = files_handler::read_datai(&filepath);
                        self.datatype = 1;
                    }
                }

                3 => {
                    // Wyswietlanie wczytanych danych
                    if self.datatype == 0 {
                        println!("Załadowane dane testowe:\n{:?}", self.dataf.clone());
                    } else if self.datatype == 1 {
                        println!("Załadowane dane testowe:\n{:?}", self.datai.clone());
                    } else {
                        println!("{}", "[-] Brak załadowanych danych!".red());
                    }
                }

                4 => {
                    // Sprawdz czy posiada wgrany zestaw danych
                    if self.datai.len() == 0 && self.dataf.len() == 0 {
                        println!(
                            "{}",
                            "[-] Błąd: Brak wczytanych danych! Proszę wczytać dane...".red()
                        );
                    } else {
                        // Wybor algorytmu sortowania
                        println!("Dostępne algorytmy:\n 1. Heap Sort.\n 2. Insert Sort.\n 3. Quick Sort.\n 4. Shell Sort.\n 5. Cofnij");
                        self.pretty("wybor-algorytmu".to_string());
                        let alg_choice = self.read_user_choice_u8();
                        match alg_choice {
                            1 => {
                                println!("{}", "[+] Uruchamianie algorytmu Heap Sort...");
                                if self.datatype == 0 {
                                    heap_sort::run(self.dataf.clone(), self.benchmarking);
                                } else if self.datatype == 1 {
                                    heap_sort::run(self.datai.clone(), self.benchmarking);
                                } else {
                                    println!("{}", "[-] Error: Datatype not chosen!".red());
                                }
                            }

                            2 => {
                                println!("{}", "[+] Uruchamianie algorytmu Insert Sort...");
                                if self.datatype == 0 {
                                    insert_sort::run(self.dataf.clone(), self.benchmarking);
                                } else if self.datatype == 1 {
                                    insert_sort::run(self.datai.clone(), self.benchmarking);
                                } else {
                                    println!("{}", "[-] Error: Datatype not chosen!".red());
                                }
                            }

                            3 => {
                                // Wybor pivota
                                println!("Wybor pivota (podaj wartość pivota):\n Lewy - 0\n Prawy - {}\n Srodek - {}\n Losowy - podanie losowej liczby <= {}", 
                                    cmp::max(self.dataf.len(), self.datai.len()) - 1,
                                    cmp::max(self.dataf.len(), self.datai.len())/2 - 1,
                                    cmp::max(self.dataf.len(), self.datai.len()) - 1,
                                );
                                self.pretty("wybor-pivota-quick-sort".to_string());

                                let pivot_index = self.read_user_choice_i64();
                                println!("{}", "[+] uruchamianie algorytmu Quick Sort...");
                                if self.datatype == 0 {
                                    quick_sort::run(
                                        self.dataf.clone(),
                                        pivot_index.try_into().unwrap(),
                                        self.benchmarking,
                                    );
                                } else if self.datatype == 1 {
                                    quick_sort::run(
                                        self.datai.clone(),
                                        pivot_index.try_into().unwrap(),
                                        self.benchmarking,
                                    );
                                } else {
                                    println!("{}", "[-] Error: Datatype not chosen!".red());
                                }
                            }

                            4 => {
                                // Wczytanie zmiennej
                                println!("Wybor algorytmu shella:\n 1. Ciąg Marcina Ciury (A102549)\n 2. Ciąg Knuth'a (A003462)");
                                self.pretty("wybor-algorytmu-shella".to_string());

                                let choice = self.read_user_choice_i64();
                                if choice > 3 && choice < 1 {
                                    println!("[-] Out of range...");
                                    break;
                                }

                                println!("{}", "[+] Uruchamianie algorytmu Shell Sort...");
                                if self.datatype == 0 {
                                    shell_sort::run(self.dataf.clone(), choice, self.benchmarking);
                                } else if self.datatype == 1 {
                                    shell_sort::run(self.datai.clone(), choice, self.benchmarking);
                                } else {
                                    println!("{}", "[-] Error: Datatype not chosen!".red());
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }

                5 => {
                    // Generowanie danych
                    self.pretty("podaj-rozmiar-tablicy".to_string());
                    let choice_local = self.read_user_choice_i64();

                    self.pretty("podaj-nazwe-pliku".to_string());
                    let filename_tmp = self.read_user_choice_str();

                    self.pretty("posortowana w ilu procentach 0->33%, 1->66%, 2->0%".to_string());
                    let sort_part: i64 = self.read_user_choice_i64();

                    if sort_part != 0 && sort_part != 1 && sort_part != 2 {
                        println!("{}", "[-] Different from 0, 1 or 2. Wrong value provided.");
                    } else {
                        if self.datatype == 0 {
                            self.dataf = generator::generate_data_f64(
                                choice_local.try_into().unwrap(),
                                filename_tmp,
                            );
                            self.dataf = generator::sort_percent(
                                self.dataf.clone(),
                                sort_part.try_into().unwrap(),
                            );
                        } else if self.datatype == 1 {
                            self.datai = generator::generate_data_i64(
                                choice_local.try_into().unwrap(),
                                filename_tmp,
                            );
                            self.datai = generator::sort_percent(
                                self.datai.clone(),
                                sort_part.try_into().unwrap(),
                            );
                        } else {
                            println!("{}", "[-] Error: Datatype not chosen!".red());
                        }
                    }
                }

                _ => return,
            }
        }
    }

    fn read_user_choice_u8(&self) -> u8 {
        let mut tmp: String = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to read user's input");

        let tmp_u8 = match tmp.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to read user's input!");
                10
            }
        };

        tmp_u8
    }

    fn read_user_choice_i64(&self) -> i64 {
        let mut tmp: String = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to read user's input");

        let tmp_i64 = match tmp.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to read user's input!");
                10
            }
        };

        tmp_i64
    }

    fn read_user_choice_str(&self) -> String {
        let mut tmp: String = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to read user's input");

        tmp.trim().to_string()
    }

    fn pretty(&self, s: String) {
        print!("{}{}{}", "projekt-1@aizo: ".green(), s.blue(), "$ ".blue());
        let _ = std::io::stdout().flush();
    }
}

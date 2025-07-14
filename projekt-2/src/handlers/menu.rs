use std::io;
use std::io::Write;
use std::path::Path;

use crate::algorithms::mfp::fulkerson;
use crate::algorithms::mst::{kruskal, prim};

use crate::algorithms::spf::{bellman, dijkstra};
use crate::handlers::{self, files_handler};
use crate::utils::{convert, generator};

use owo_colors::OwoColorize;

#[derive(Clone)]
pub struct Menu {
    datai: Vec<i64>,
    dataf: Vec<f64>,
    datamst: Vec<Vec<i64>>,
    dataspf: Vec<Vec<i64>>,
    datamfp: Vec<Vec<i64>>,
    benchmarking: bool,
}
impl Menu {
    pub fn new(
        datai: Vec<i64>,
        dataf: Vec<f64>,
        datamst: Vec<Vec<i64>>,
        dataspf: Vec<Vec<i64>>,
        datamfp: Vec<Vec<i64>>,
        benchmarking: bool,
    ) -> Menu {
        Menu {
            datai,
            dataf,
            datamst,
            dataspf,
            datamfp,
            benchmarking,
        }
    }

    pub fn run(&mut self) {
        let mut user_choice: u8;
        loop {
            // Pierwszy poziom menu
            println!("Menu:\n 1. Wczytanie pliku z danymi.\n 2. Wyswietlanie wczytanych danych z pliku.\n 3. Wybór problemu.\n 4. Generowanie danych.\n 5. Zamknij program.");
            self.pretty(String::from("~"));
            user_choice = self.read_user_choice_u8();

            match user_choice {
                1 => {
                    // Wybranie wariantu pliku w zaleznosci od problemu
                    println!(
                        "Wybor wariantu plikow ze wgledu na problem:\n 1. MFP\n 2. MST\n 3. SPF"
                    );
                    self.pretty("wybor-wariantu-ze-wzgledu-na-problem".to_string());
                    // Wczytanie danych z pliku
                    let problem_choice: u8 = self.read_user_choice_u8();

                    match problem_choice {
                        1 => {
                            // MFP

                            // Wybor pliku z danymi
                            files_handler::list_files("mfp".to_string());
                            self.pretty("wybor-pliku".to_string());
                            let chosen_file = self.read_user_choice_str();
                            let filepath = Path::new("./data/mfp/").join(chosen_file);
                            self.datamfp = files_handler::read_data_mpf(&filepath);
                            // self.datai = files_handler::read_datai(&filepath);
                        }

                        2 => {
                            // MST

                            // Wybor pliku z danymi
                            files_handler::list_files("mst".to_string());
                            self.pretty("wybor-pliku".to_string());
                            let chosen_file = self.read_user_choice_str();
                            let filepath = Path::new("./data/mst/").join(chosen_file);
                            self.datamst = files_handler::read_data_mst(&filepath);
                        }

                        3 => {
                            // SFP - to do

                            // Wybor pliku z danymi
                            files_handler::list_files("spf".to_string());
                            self.pretty("wybor-pliku".to_string());
                            let chosen_file = self.read_user_choice_str();
                            let filepath = Path::new("./data/spf/").join(chosen_file);
                            self.dataspf = files_handler::read_data_spf(&filepath);
                        }

                        _ => {
                            continue;
                        }
                    }
                }

                2 => {
                    // Wyswietlanie wczytanych danych
                    if self.dataspf.len() != 0 || self.datamst.len() != 0 || self.datamfp.len() != 0
                    {
                        println!("Data MFP");
                        Menu::printvec(self.datamfp.clone());
                        println!("Data MST");
                        Menu::printvec(self.datamst.clone());
                        println!("Data SPF");
                        Menu::printvec(self.dataspf.clone());
                    } else {
                        println!("{}", "[-] Brak załadowanych danych!".red());
                    }
                }

                3 => {
                    // Sprawdz czy posiada wgrany zestaw danych
                    if self.datamfp.len() == 0 && self.dataspf.len() == 0 && self.datamst.len() == 0
                    {
                        println!(
                            "{}",
                            "[-] Błąd: Brak wczytanych danych! Proszę wczytać dane...".red()
                        );
                    } else {
                        // Wybor algorytmu sortowania
                        println!("Dostępne problemy:\n 1. MFP.\n 2. MST.\n 3. SPF.\n 4. Cofnij");
                        self.pretty("wybor-problemu".to_string());
                        let problem_choice: u8 = self.read_user_choice_u8();
                        match problem_choice {
                            1 => {
                                // MFP - to do
                                println!(
                                    "Dostepne algorytmy:\n 1. Algorytm Fulkersona.\n 3. Cofnij."
                                );
                                self.pretty("wybor-algorytmu".to_string());
                                let alg_choice: u8 = self.read_user_choice_u8();
                                match alg_choice {
                                    1 => {
                                        println!("[+] Uruchamianie algorytmu Fulkersona...");
                                        fulkerson::run(self.datamfp.clone(), 1);
                                        fulkerson::run(self.datamfp.clone(), 2);
                                        // kruskal::run(self.datamst.clone());
                                    }

                                    _ => {
                                        continue;
                                    }
                                }
                            }

                            2 => {
                                // MST
                                println!("Dostepne algorytmy:\n 1. Algorytm Kruskala.\n 2. Algorytm Prima.\n 3. Cofnij.");
                                self.pretty("wybor-algorytmu".to_string());
                                let alg_choice: u8 = self.read_user_choice_u8();
                                match alg_choice {
                                    1 => {
                                        println!("[+] Uruchamianie algorytmu Kruskala...");
                                        kruskal::run(self.datamst.clone(), 1);
                                        kruskal::run(self.datamst.clone(), 2);
                                        // kruskal::run(self.datamst.clone());
                                    }

                                    2 => {
                                        println!("[+] Uruchamianie algorytmu Prima...");
                                        prim::run(self.datamst.clone(), 1);
                                        prim::run(self.datamst.clone(), 2);
                                    }

                                    _ => {
                                        continue;
                                    }
                                }
                            }

                            3 => {
                                // SFP - to do
                                println!("Dostepne algorytmy:\n 1. Algorytm Dijkstry.\n 2. Algorytm Bellmana.\n 3. Cofnij.");
                                self.pretty("wybor-algorytmu".to_string());
                                let alg_choice: u8 = self.read_user_choice_u8();
                                match alg_choice {
                                    1 => {
                                        println!("[+] Uruchamianie algorytmu Dijkstry...");
                                        dijkstra::run(self.dataspf.clone(), 1);
                                        dijkstra::run(self.dataspf.clone(), 2);
                                    }

                                    2 => {
                                        println!("[+] Uruchamianie algorytmu Bellmana...");
                                        bellman::run(self.dataspf.clone(), 1);
                                        bellman::run(self.dataspf.clone(), 2);
                                    }

                                    _ => {
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }

                4 => {
                    // Wybor generowanego grafu
                    println!("Dostepne algorytmy:\n 1. MST.\n 2. MFP.\n 3. SPF.\n 4. Cofnij.");
                    self.pretty("wybor-algorytmu".to_string());

                    let problem_choice: u8 = self.read_user_choice_u8();

                    match problem_choice {
                        1 => {
                            // MST
                            println!("Proszę o podanie ilości wierzchołków:");
                            let verticles: i64 = self.read_user_choice_i64();

                            println!("Proszę o podanie gęstości grafu:");
                            let density: i64 = self.read_user_choice_i64();

                            self.datamst = generator::generate_graph_mst(verticles, density);
                        }

                        2 => {
                            println!("Proszę o podanie ilości wierzchołków:");
                            let verticles: i64 = self.read_user_choice_i64();

                            println!("Proszę o podanie gęstości grafu:");
                            let density: i64 = self.read_user_choice_i64();

                            self.datamfp = generator::generate_graph_mfp(verticles, density);
                        }

                        3 => {
                            println!("Proszę o podanie ilości wierzchołków:");
                            let verticles: i64 = self.read_user_choice_i64();

                            println!("Proszę o podanie gęstości grafu:");
                            let density: i64 = self.read_user_choice_i64();

                            self.dataspf = generator::generate_graph_spf(verticles, density);
                        }

                        _ => {
                            continue;
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

    fn printvec(data: Vec<Vec<i64>>) {
        for i in data {
            println!("{:?}", i);
        }
    }
}

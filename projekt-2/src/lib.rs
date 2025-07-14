use clap::Parser;

/// Projekt 1 - Badanie efektywności wybranych algorytmów sortowania ze względu na złożoność obliczeniową.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    /// Benchmarking
    #[arg(short, long, default_value_t = false)]
    pub benchmark_run: bool,

    /// Menu
    #[arg(short, long, default_value_t = false)]
    pub menu_run: bool,
}

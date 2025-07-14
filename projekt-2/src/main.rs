mod algorithms;
mod handlers;
mod utils;

use clap::Parser;
use handlers::user_handler::UserHandler;
use projekt_1::Args;

fn main() {
    let args = Args::parse();
    let userhandler: UserHandler = UserHandler::new(args.menu_run, args.benchmark_run);
    userhandler.run();
}

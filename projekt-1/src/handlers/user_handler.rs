use crate::handlers;

use handlers::menu::Menu;

pub struct UserHandler {
    menu_run: bool,
    benchmark_run: bool,
}
impl UserHandler {
    pub fn new(menu_run: bool, benchmark_run: bool) -> UserHandler {
        UserHandler {
            menu_run,
            benchmark_run,
        }
    }

    pub fn run(self) {
        if self.menu_run {
            // Menu
            let mut usermenu: Menu = Menu::new(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                self.benchmark_run,
            );
            usermenu.run();
        }
    }
}

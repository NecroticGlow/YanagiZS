pub mod config;
pub mod db_const;
pub mod logging;
pub mod time_util;

pub enum ServiceStatus {
    StopServer(u64),
    Running,
}

pub fn print_splash() {
    println!("  ______         _               _____   _____ \n |  ____|       | |             |  __ \\ / ____|\n | |____   _____| |_   _ _ __   | |__) | (___  \n |  __\\ \\ / / _ \\ | | | | '_ \\  |  _  / \\___ \\ \n | |___\\ V /  __/ | |_| | | | | | | \\ \\ ____) |\n |______\\_/ \\___|_|\\__, |_| |_| |_|  \\_\\_____/ \n                    __/ |                      \n                   |___/                       ");
}

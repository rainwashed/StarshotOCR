// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;


fn main() {
    println!("Welcome to Starshot.");
    starshot_lib::run();
}

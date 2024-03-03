// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
use crate::error::Error;

mod db;
use crate::db::{query, DbCxn};

mod model;
mod utils;
use crate::utils::read_file;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello from Rust, {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let filename = "bases\\test.db";
    let client = DbCxn::init(filename).await?;

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(client)
        .invoke_handler(tauri::generate_handler![greet, query, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn startup(name: &str) -> String {
  println!("executing handle_button1");
  format!("Hello, {}!", name)
}


#[tauri::command]
fn handle_left_button(name: &str) -> String {
  println!("executing handle_button1");
  format!("Hello, {}!", name)
}


#[tauri::command]
fn handle_right_button(name: &str) -> String {
  println!("exec    uting handle_button2");
  format!("Hello, {}!", name)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![startup, handle_left_button, handle_right_button])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  }
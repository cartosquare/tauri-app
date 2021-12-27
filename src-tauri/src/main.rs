#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::hello_world_test,
      cmd::render_map,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

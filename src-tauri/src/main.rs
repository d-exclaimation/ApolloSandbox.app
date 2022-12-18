#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn ok() -> String {
  String::from("All goods here 👍")
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![ok])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

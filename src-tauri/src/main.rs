#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
}

async fn something_else() -> Option<String> {
  Some("All goods from the frontend 👍".into())
}

#[tauri::command]
async fn something(
  window: tauri::Window
) -> Result<CustomResponse, String> {
  println!("All goods here 👍 at native level, passing back to {}", window.label());
  let result: Option<String> = something_else().await;
  if let Some(message) = result {
    Ok(CustomResponse {
      message
    })
  } else {
    Err("No result".into())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![something])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
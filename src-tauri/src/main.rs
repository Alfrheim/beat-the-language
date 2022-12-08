#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_random_word])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_random_word(language: &str) -> String {
    match language {
        "EN" => String::from("House"),
        _ => String::from("couldn't recover the word")
    }
}
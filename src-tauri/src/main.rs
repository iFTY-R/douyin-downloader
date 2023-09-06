#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod command;
mod menu;
mod douyin_api;
mod proxy_request;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      command::show,
      command::exists,
      command::download,
      command::disk_free_size,
      command::create_dir_recursive,
      douyin_api::get_user_info,
      proxy_request::proxy_request,
    ])
    .menu(menu::generate_menu())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

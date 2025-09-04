pub mod api;

use actix_web;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::thread::spawn(|| {
        actix_web::rt::System::new().block_on(async{
            api::server::start_server().await.unwrap();
        });
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
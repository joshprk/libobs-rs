use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use libobs_wrapper::{
    bootstrap::{status_handler::ObsBootstrapConsoleHandler, ObsBootstrapperOptions},
    runtime::{ObsRuntime, ObsRuntimeReturn},
};
use tauri::async_runtime::block_on;

lazy_static! {
    pub static ref OBS_RUNTIME: Arc<RwLock<Option<ObsRuntime>>> = Arc::new(RwLock::new(None));
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(_name: String) -> String {
    // Get a clone of the runtime to avoid holding the lock across the await
    let runtime_clone = {
        let runtime_guard = OBS_RUNTIME.read().unwrap();
        runtime_guard.as_ref().map(|r| r.clone())
    };

    // Use the cloned runtime, which is now outside the lock scope
    if let Some(runtime) = runtime_clone {
        let scene_len = runtime
            .run_with_obs_result(|ctx| Ok(ctx.scenes().borrow().len()))
            .await;

        format!("Hi, this is OBS with a total of {:?} scenes", scene_len)
    } else {
        "OBS runtime is not initialized".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let f = ObsRuntime::new()
                .enable_bootstrapper(
                    ObsBootstrapConsoleHandler,
                    ObsBootstrapperOptions::default(),
                )
                .start();

            let r = block_on(f).expect("Failed to start OBS runtime");
            match r {
                ObsRuntimeReturn::Done(r) => {
                    OBS_RUNTIME.write().unwrap().replace(r);
                }
                ObsRuntimeReturn::Restart => {
                    app.handle().exit(0);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

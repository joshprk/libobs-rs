use std::sync::{atomic::AtomicBool, Arc, RwLock};

use bootstrap_status::ObsTauriStatusHandler;
use lazy_static::lazy_static;
use libobs_wrapper::{
    bootstrap::ObsBootstrapperOptions,
    runtime::{ObsRuntime, ObsRuntimeReturn},
};
use tauri::{AppHandle, Emitter};
mod bootstrap_status;

lazy_static! {
    pub static ref IS_BOOTSTRAPPING: AtomicBool = AtomicBool::new(false);
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

#[tauri::command]
async fn bootstrap(handle: AppHandle) -> String {
    if OBS_RUNTIME.read().unwrap().is_some() {
        return "OBS is already running.".to_string();
    }

    if IS_BOOTSTRAPPING.load(std::sync::atomic::Ordering::SeqCst) {
        return "OBS is already starting.".to_string();
    }

    IS_BOOTSTRAPPING.store(true, std::sync::atomic::Ordering::SeqCst);
    let f = ObsRuntime::new()
        .enable_bootstrapper(
            ObsTauriStatusHandler {
                handle: handle.clone(),
            },
            ObsBootstrapperOptions::default(),
        )
        .start();

    let r = f.await.expect("Failed to start OBS runtime");
    match r {
        ObsRuntimeReturn::Done(r) => {
            println!("Setting runtime to runtime");
            OBS_RUNTIME.write().unwrap().replace(r);
        }
        ObsRuntimeReturn::Restart => {
            println!("Restarting OBS runtime");
            handle.exit(0);
        }
    }

    println!("Runtime done");
    handle.emit("bootstrap_done", ()).unwrap();

    IS_BOOTSTRAPPING.store(false, std::sync::atomic::Ordering::SeqCst);
    "Done.".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, bootstrap])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

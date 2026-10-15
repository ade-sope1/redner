use uuid::Uuid;
use crate::core::api::RednerApi;
use crate::core::engine::RednerCoreRuntime;

// ==========================================
// TAURI IPC COMMANDS (DESKTOP BRIDGE)
// These functions are exposed directly to the React frontend.
// ==========================================

/// This simulates a `#[tauri::command]` that the React UI calls via `invoke('create_local_project')`
pub fn tauri_create_project(
    runtime: &RednerCoreRuntime, 
    title: String, 
    author_id: Uuid
) -> Result<Uuid, String> {
    
    // 1. The React app called us. We connect to the Universal API.
    let api = RednerApi::connect(runtime, "desktop-tauri");
    
    // 2. We execute the command. The UI knows nothing about the underlying engine.
    api.create_project(&title, author_id)
}

/// Simulates a `#[tauri::command]` for dropping a heavy local 4K file into the app
pub fn tauri_import_local_file(
    runtime: &RednerCoreRuntime, 
    project_id: Uuid, 
    file_path: String
) -> Result<Uuid, String> {
    
    let api = RednerApi::connect(runtime, "desktop-tauri");
    
    // In a real desktop app, we would use Rust's `std::fs` to read the heavy 4K file directly 
    // from the user's hard drive here, bypassing browser upload limits entirely!
    println!("💽 [TAURI] Reading massive local file directly from disk: {}", file_path);
    
    api.add_asset(project_id, "Local 4K Video", &file_path)
}
use uuid::Uuid;
use crate::core::api::RednerApi;
use crate::core::engine::RednerCoreRuntime;

// ==========================================
// 1. THE WEB SURFACE (e.g., Axum HTTP Handler)
// Represents the Redner browser application.
// ==========================================
pub struct WebSurface;

impl WebSurface {
    pub fn handle_http_request(runtime: &RednerCoreRuntime, user_id: Uuid) {
        println!("\n🌐 [WEB CLIENT] User clicked 'New Project' in the browser...");
        
        // Connect to the Facade
        let api = RednerApi::connect(runtime, "web-axum-client");
        
        // Execute the action
        let _ = api.create_project("Browser Edited Vlog", user_id);
    }
}

// ==========================================
// 2. THE DESKTOP SURFACE (e.g., Tauri IPC Command)
// Represents the installed native Redner application.
// ==========================================
pub struct DesktopSurface;

impl DesktopSurface {
    pub fn handle_ipc_command(runtime: &RednerCoreRuntime, user_id: Uuid) {
        println!("\n💻 [DESKTOP CLIENT] User dropped a video file into the native app...");
        
        // Connect to the Facade
        let api = RednerApi::connect(runtime, "desktop-tauri-client");
        
        // Execute the action (Notice how the API usage is identical to the Web surface!)
        if let Ok(project_id) = api.create_project("Desktop Heavy Render", user_id) {
            let _ = api.add_asset(project_id, "Raw 4K Footage", "file:///C:/videos/raw.mp4");
        }
    }
}

// ==========================================
// 3. THE CLI SURFACE (e.g., Headless Automation)
// Represents a developer running `redner build` in their terminal.
// ==========================================
pub struct CliSurface;

impl CliSurface {
    pub fn handle_terminal_command(runtime: &RednerCoreRuntime, user_id: Uuid) {
        println!("\n🖥️  [CLI CLIENT] Developer triggered a headless automation script...");
        
        // Connect to the Facade
        let api = RednerApi::connect(runtime, "headless-cli");
        
        // Execute the action
        if let Ok(project_id) = api.create_project("Automated Shorts Pipeline", user_id) {
            let _ = api.run_agent(project_id, "TikTok Auto-Editor", "Chop this into 3 viral shorts");
        }
    }
}

pub async fn run_multi_surface_demo() {
    println!("\n🚀 BOOTING REDNER OS MULTI-SURFACE TEST...");
    
    // Boot the single, unified Core Engine
    let global_runtime = RednerCoreRuntime::init();
    let test_user_id = Uuid::new_v4();

    // Simulate traffic hitting the engine from completely different surfaces
    WebSurface::handle_http_request(&global_runtime, test_user_id);
    DesktopSurface::handle_ipc_command(&global_runtime, test_user_id);
    CliSurface::handle_terminal_command(&global_runtime, test_user_id);
    
    println!("✅ REDNER OS MULTI-SURFACE TEST COMPLETE.\n");
}


use crate::core::command_center::CommandCenter;
use crate::core::integration::{OsIntegrationLayer, ExternalPlatform};

pub async fn run_redner_os_grand_finale() {
    println!("\n👑 BOOTING REDNER OS: THE GRAND FINALE...");
    
    let global_runtime = RednerCoreRuntime::init();
    let creator_id = Uuid::new_v4();

    // 1. The Creator uses the Global Command Palette (14.9)
    let _ = CommandCenter::process_query(&global_runtime, creator_id, "Prepare my latest video for TikTok");

    // 2. The Orchestrator (14.8) and Universal Agent (14.7) handle the timeline assembly via the Core API (14.2)
    // 3. The final MP4 is generated. Now, the OS integrates with the outside world (14.10)
    
    let simulated_project_id = Uuid::new_v4();
    let _ = OsIntegrationLayer::publish_artifact(
        &global_runtime, 
        simulated_project_id, 
        ExternalPlatform::TikTok, 
        "./exports/viral_short_final.mp4"
    );

    let _ = OsIntegrationLayer::publish_artifact(
        &global_runtime, 
        simulated_project_id, 
        ExternalPlatform::Substack, 
        "Building a Custom Engine in Rust"
    );

    println!("\n✅ REDNER OS IS FULLY OPERATIONAL.");
}
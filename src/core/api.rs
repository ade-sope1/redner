use uuid::Uuid;
use crate::core::engine::RednerCoreRuntime;
use crate::core::command::{CoreCommand, CoreExecutionResult};

// ==========================================
// THE UNIVERSAL REDNER API (FACADE)
// The single high-level interface for all client surfaces.
// ==========================================
pub struct RednerApi<'a> {
    runtime: &'a RednerCoreRuntime,
    surface_identifier: String, // e.g., "desktop-tauri", "web-axum", "cli"
}

impl<'a> RednerApi<'a> {
    /// Connects a client surface to the Redner Core.
    pub fn connect(runtime: &'a RednerCoreRuntime, surface: &str) -> Self {
        println!("🔌 [API] Surface '{}' connected to Redner Core.", surface);
        Self {
            runtime,
            surface_identifier: surface.to_string(),
        }
    }

    /// High-level wrapper for Project Creation
    pub fn create_project(&self, title: &str, author_id: Uuid) -> Result<Uuid, String> {
        let result = self.execute_command(CoreCommand::CreateProject {
            title: title.to_string(),
            author_id,
        })?;
        
        result.affected_entity_id.ok_or_else(|| "Failed to retrieve project ID".to_string())
    }

    /// High-level wrapper for Asset Ingestion
    pub fn add_asset(&self, project_id: Uuid, name: &str, source_url: &str) -> Result<Uuid, String> {
        let result = self.execute_command(CoreCommand::AddAsset {
            project_id,
            name: name.to_string(),
            source_url: source_url.to_string(),
        })?;

        result.affected_entity_id.ok_or_else(|| "Failed to retrieve asset ID".to_string())
    }

    /// High-level wrapper for Timeline Editing
    pub fn insert_clip(&self, project_id: Uuid, track_id: Uuid, asset_id: &str, start_ms: u64, duration_ms: u64) -> Result<Uuid, String> {
        let result = self.execute_command(CoreCommand::InsertClip {
            project_id,
            track_id,
            asset_id: asset_id.to_string(),
            start_ms,
            duration_ms,
        })?;

        result.affected_entity_id.ok_or_else(|| "Failed to retrieve clip ID".to_string())
    }

    /// High-level wrapper for AI Workflow Automation
    pub fn run_agent(&self, project_id: Uuid, agent_id: &str, instruction: &str) -> Result<(), String> {
        self.execute_command(CoreCommand::DispatchAgentJob {
            project_id,
            agent_id: agent_id.to_string(),
            instruction: instruction.to_string(),
        })?;
        Ok(())
    }

    // --- INTERNAL ROUTER ---
    fn execute_command(&self, command: CoreCommand) -> Result<CoreExecutionResult, String> {
        println!("📡 [API -> CORE] Routing command from surface: {}", self.surface_identifier);
        self.runtime.execute(command)
    }
}
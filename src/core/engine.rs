use uuid::Uuid;
use crate::core::command::{CoreCommand, CoreExecutionResult};

// ==========================================
// THE REDNER CORE RUNTIME
// The headless, surface-agnostic master engine.
// ==========================================
pub struct RednerCoreRuntime {
    pub instance_id: Uuid,
}

impl RednerCoreRuntime {
    /// Bootstraps an instance of the Core Runtime.
    pub fn init() -> Self {
        let instance_id = Uuid::new_v4();
        println!("⚡ [REDNER CORE] Initializing Unified Core Runtime (Instance: {})", instance_id);
        Self { instance_id }
    }

    /// The single entry point for command execution regardless of surface.
    pub fn execute(&self, command: CoreCommand) -> Result<CoreExecutionResult, String> {
        let execution_id = Uuid::new_v4();

        match command {
            CoreCommand::CreateProject { title, author_id } => {
                let project_id = Uuid::new_v4();
                println!("🎬 [REDNER CORE] Initialized Project '{}' for Author {}", title, author_id);
                Ok(CoreExecutionResult {
                    execution_id,
                    success: true,
                    message: format!("Project '{}' created successfully.", title),
                    affected_entity_id: Some(project_id),
                })
            }
            CoreCommand::AddAsset { name, source_url, .. } => {
                let asset_id = Uuid::new_v4();
                println!("📦 [REDNER CORE] Asset '{}' ingested from '{}'", name, source_url);
                Ok(CoreExecutionResult {
                    execution_id,
                    success: true,
                    message: format!("Asset '{}' added.", name),
                    affected_entity_id: Some(asset_id),
                })
            }
            CoreCommand::InsertClip { start_ms, .. } => {
                let clip_id = Uuid::new_v4();
                println!("🎞️ [REDNER CORE] Clip inserted at {}ms.", start_ms);
                Ok(CoreExecutionResult {
                    execution_id,
                    success: true,
                    message: "Clip inserted.".to_string(),
                    affected_entity_id: Some(clip_id),
                })
            }
            CoreCommand::DispatchAgentJob { agent_id, instruction, .. } => {
                println!("🤖 [REDNER CORE] Dispatched Agent '{}': \"{}\"", agent_id, instruction);
                Ok(CoreExecutionResult {
                    execution_id,
                    success: true,
                    message: format!("Agent '{}' dispatched.", agent_id),
                    affected_entity_id: None,
                })
            }
        }
    }
}
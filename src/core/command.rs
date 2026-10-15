use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. UNIFIED CORE COMMANDS
// The canonical instruction set for Redner.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CoreCommand {
    CreateProject { title: String, author_id: Uuid },
    AddAsset { project_id: Uuid, name: String, source_url: String },
    InsertClip { project_id: Uuid, track_id: Uuid, asset_id: String, start_ms: u64, duration_ms: u64 },
    DispatchAgentJob { project_id: Uuid, agent_id: String, instruction: String },
}

// ==========================================
// 2. UNIFIED CORE EXECUTION RESPONSE
// What every client surface receives back.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoreExecutionResult {
    pub execution_id: Uuid,
    pub success: bool,
    pub message: String,
    pub affected_entity_id: Option<Uuid>,
}
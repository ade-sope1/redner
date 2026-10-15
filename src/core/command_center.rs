use uuid::Uuid;
use crate::core::engine::RednerCoreRuntime;
use crate::core::automation::WorkflowOrchestrator;
use crate::core::sync::HybridSyncEngine;

// ==========================================
// THE REDNER COMMAND CENTER
// Omnipresent control surface translating natural language to OS execution.
// ==========================================
pub struct CommandCenter;

impl CommandCenter {
    /// Parses a natural language query and maps it to a system-level execution plan.
    pub fn process_query(runtime: &RednerCoreRuntime, _user_id: Uuid, query: &str) -> Result<(), String> {
        println!("\n🎛️ [COMMAND CENTER] Received Query: \"{}\"", query);

        let lower_query = query.to_lowercase();

        // Basic intent parsing (In production, this routes through your AI Intelligence Layer)
        if lower_query.contains("prepare") && (lower_query.contains("tiktok") || lower_query.contains("short")) {
            println!("   ├─ Intent Parsed: Trigger Viral Shorts Pipeline");
            
            // 1. Automatically create a project container via the OS
            let project_id = Uuid::new_v4();
            println!("   ├─ Context Builder: Created new workspace (ID: {})", project_id);

            // 2. Build the workflow blueprint
            let workflow = WorkflowOrchestrator::build_viral_shorts_pipeline(project_id);

            // 3. Preview plan for the user interface
            WorkflowOrchestrator::preview_plan(&workflow);

            // 4. Auto-execute (Simulating the user hitting 'Enter' on the plan)
            println!("   ├─ Execution: User approved. Dispatching to OS Orchestrator...");
            WorkflowOrchestrator::execute(runtime, &workflow)?;

        } else if lower_query.contains("sync") {
            println!("   ├─ Intent Parsed: Trigger Hybrid Cloud Synchronization");
            println!("   └─ Execution: Dispatching to SyncEngine...");
            
            let project_id = Uuid::new_v4();
            HybridSyncEngine::sync_project_state(project_id, vec![]);
        } else {
            println!("   └─ Intent Unknown: Fallback to conversational AI agent...");
        }

        Ok(())
    }
}
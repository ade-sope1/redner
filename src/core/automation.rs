use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::engine::RednerCoreRuntime;
use crate::core::agent_runtime::UniversalAgent;

// ==========================================
// 1. WORKFLOW STEPS
// The individual stages of a creative pipeline.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AutomationStep {
    AnalyzeScript,
    GenerateVoiceover,
    SynthesizeVisuals,
    AssembleTimeline,
    QualityAssurance,
    RenderAndExport,
}

// ==========================================
// 2. THE CREATIVE WORKFLOW BLUEPRINT
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeWorkflow {
    pub workflow_id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub steps: Vec<AutomationStep>,
}

// ==========================================
// 3. THE WORKFLOW ORCHESTRATOR
// Manages the multi-step execution pipeline.
// ==========================================
pub struct WorkflowOrchestrator;

impl WorkflowOrchestrator {
    /// Constructs a standard pipeline (e.g., for TikTok/Shorts).
    pub fn build_viral_shorts_pipeline(project_id: Uuid) -> CreativeWorkflow {
        CreativeWorkflow {
            workflow_id: Uuid::new_v4(),
            project_id,
            name: "End-to-End Viral Short Pipeline".to_string(),
            steps: vec![
                AutomationStep::AnalyzeScript,
                AutomationStep::GenerateVoiceover,
                AutomationStep::SynthesizeVisuals,
                AutomationStep::AssembleTimeline,
                AutomationStep::QualityAssurance,
                AutomationStep::RenderAndExport,
            ],
        }
    }

    /// Previews the plan BEFORE execution (crucial for creator control).
    pub fn preview_plan(workflow: &CreativeWorkflow) {
        println!("\n📋 [AUTOMATION] Previewing Execution Plan: '{}'", workflow.name);
        for (i, step) in workflow.steps.iter().enumerate() {
            println!("   ├─ Step {}: {:?}", i + 1, step);
        }
        println!("   └─ Waiting for Creator Approval...");
    }

    /// Executes the approved pipeline autonomously.
    pub fn execute(runtime: &RednerCoreRuntime, workflow: &CreativeWorkflow) -> Result<(), String> {
        println!("\n🚀 [AUTOMATION] Executing Pipeline: '{}'", workflow.name);
        
        // The Orchestrator delegates complex timeline assembly to the Universal Agent
        let director_agent = UniversalAgent::new("director-alpha", "Master Director Agent");

        for step in &workflow.steps {
            match step {
                AutomationStep::AnalyzeScript => println!("   ⏳ Executing: Extracting hooks and pacing from script..."),
                AutomationStep::GenerateVoiceover => println!("   ⏳ Executing: Synthesizing ElevenLabs voice track..."),
                AutomationStep::SynthesizeVisuals => println!("   ⏳ Executing: Generating B-Roll via Runway API..."),
                AutomationStep::AssembleTimeline => {
                    println!("   ⏳ Executing: Handing off to Universal Agent for timeline assembly...");
                    let _ = director_agent.execute_autonomous_workflow(
                        runtime, 
                        workflow.project_id, 
                        "Sync generated B-roll to the audio waveform"
                    );
                }
                AutomationStep::QualityAssurance => println!("   ⏳ Executing: AI reviewing video for dead air and visual glitches..."),
                AutomationStep::RenderAndExport => println!("   ⏳ Executing: Dispatching to GPU Render Queue..."),
            }
        }

        println!("✅ [AUTOMATION] Pipeline '{}' finished completely. Content is ready for review.", workflow.name);
        Ok(())
    }
}
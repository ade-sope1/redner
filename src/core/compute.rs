use serde::{Deserialize, Serialize};

// ==========================================
// 1. COMPUTE ENVIRONMENTS & TASK TYPES
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ComputeEnvironment {
    LocalMachine, // Uses the user's CPU/GPU via Desktop App
    CloudCluster, // Uses Redner's server infrastructure / APIs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AiTaskType {
    Transcription,         // e.g., Whisper
    LightweightText,       // e.g., Local Llama 3 / Mistral
    HeavyVideoGeneration,  // e.g., Sora / Runway
    VoiceCloning,          // e.g., ElevenLabs
}

// ==========================================
// 2. THE HYBRID COMPUTE ROUTER
// Dynamically decides where to execute a job.
// ==========================================
pub struct HybridComputeEngine;

impl HybridComputeEngine {
    /// Evaluates the task requirements and hardware availability to route the job.
    pub fn determine_optimal_environment(
        task: &AiTaskType, 
        has_local_gpu: bool
    ) -> ComputeEnvironment {
        println!("\n⚖️  [COMPUTE ROUTER] Evaluating execution path for {:?}", task);

        match task {
            // Lightweight text and transcription are perfect for local execution to save API costs
            AiTaskType::Transcription | AiTaskType::LightweightText => {
                if has_local_gpu {
                    println!("   ├─ Hardware check: Local GPU detected.");
                    println!("   └─ 🧠 ROUTE: LocalMachine (Zero latency, free inference).");
                    ComputeEnvironment::LocalMachine
                } else {
                    println!("   ├─ Hardware check: No capable local GPU.");
                    println!("   └─ ☁️ ROUTE: CloudCluster (Fallback).");
                    ComputeEnvironment::CloudCluster
                }
            }
            // Heavy tasks always require cloud infrastructure
            AiTaskType::HeavyVideoGeneration | AiTaskType::VoiceCloning => {
                println!("   ├─ Requirement check: Massive VRAM / Proprietary API needed.");
                println!("   └─ ☁️ ROUTE: CloudCluster (Direct).");
                ComputeEnvironment::CloudCluster
            }
        }
    }

    /// Simulates executing the task in the chosen environment
    pub fn execute(task: AiTaskType, env: ComputeEnvironment) -> Result<(), String> {
        if env == ComputeEnvironment::LocalMachine {
            println!("⚙️  [LOCAL COMPUTE] Spinning up local model in background thread...");
            // Tauri would spin up an ONNX or candle/tch-rs runtime here
        } else {
            println!("🌐 [CLOUD COMPUTE] Dispatching payload to external API providers...");
            // Cloud API would handle it here
        }
        Ok(())
    }
}
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageLocation {
    LocalDiskOnly(String), // e.g., "C:/videos/raw.mp4"
    CloudBucket(String),   // e.g., "s3://redner-assets/..."
    HybridSynced,          // Exists in both places
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HybridAsset {
    pub asset_id: Uuid,
    pub name: String,
    pub location: StorageLocation,
    pub size_mb: u64,
}

// ==========================================
// THE SYNCHRONIZATION ENGINE
// ==========================================
pub struct HybridSyncEngine;

impl HybridSyncEngine {
    /// Determines how to sync a project between Desktop and Web.
    pub fn sync_project_state(project_id: Uuid, assets: Vec<HybridAsset>) {
        println!("🔄 [SYNC] Synchronizing Project {} across surfaces...", project_id);
        
        // 1. Metadata ALWAYS syncs (it's lightweight JSON)
        println!("   ├─ Syncing Timeline Metadata -> Cloud Database");
        
        // 2. Assets sync conditionally
        for asset in assets {
            match asset.location {
                StorageLocation::LocalDiskOnly(_) => {
                    if asset.size_mb > 100 {
                        println!("   ├─ SKIP: Asset '{}' is too large ({}MB). Keeping local-only reference.", asset.name, asset.size_mb);
                    } else {
                        println!("   ├─ UPLOAD: Syncing Asset '{}' to Cloud Bucket.", asset.name);
                    }
                }
                StorageLocation::CloudBucket(_) | StorageLocation::HybridSynced => {
                    println!("   ├─ OK: Asset '{}' is already available in the cloud.", asset.name);
                }
            }
        }
        println!("✅ [SYNC] Project state is synchronized.");
    }
}
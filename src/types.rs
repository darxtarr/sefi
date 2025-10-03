// Core data structures for Sefi v0.3 (N-D Primary)

use serde::{Deserialize, Serialize};

/// Tempo for decay and persistence behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tempo {
    Fast,   // τ=2s - bursts, alerts, quick iterations
    Slow,   // τ=30s - consensus, persistent thoughts
    Urgent, // bypass persistence, emit immediately
}

impl Tempo {
    /// Get decay time constant in seconds
    pub fn tau(&self) -> f32 {
        match self {
            Tempo::Fast => 2.0,
            Tempo::Slow => 30.0,
            Tempo::Urgent => 0.0, // bypass
        }
    }
}

/// Polarity for attraction/repulsion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Polarity {
    Attract,
    Repel,
}

/// Concept packet emitted by agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptPacket {
    pub phrase: String,          // anchor text (2-6 words)
    pub amp: f32,                // [0..1] confidence/urgency
    pub sigma: f32,              // breadth vs specificity
    pub polarity: Polarity,      // attract | repel
    pub tempo: Tempo,            // decay/persistence tempo
    pub provenance: String,      // hash/pointer to source context
    pub agent_id: String,        // stable role ID
    pub rationale_hash: String,  // hash of reasoning step
    pub timestamp: u64,          // ms epoch
}

/// Basin type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BasinType {
    Valley, // consensus
    Ridge,  // tradeoff boundary
    Peak,   // overload/contradiction
}

/// Recommended action for basin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    PlanSpike,        // valley → spawn implementation
    PairedExperiment, // ridge → probe both sides
    Decompose,        // peak → split into sub-basins
    IgnoreShortLived, // transient noise
}

/// Synthesis tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynthTier {
    Template, // instant, no LLM - just top phrases + action
    Light,    // future: lightweight LLM summary
    Heavy,    // future: full crux card with steelman
}

/// PreCard template for instant feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCard {
    pub tier: SynthTier,          // Template | Light | Heavy
    pub summary: String,          // instant template summary
    pub top_phrases: Vec<String>, // top-k contributor phrases
    pub suggested_action: String, // template action text
}

/// Threshold snapshot for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSnapshot {
    pub persistence_min: u32,
    pub density_threshold: f32,
    pub nd_min_members: usize,
    pub nd_radius: f32,
    pub window_w: u32,
}

/// Basin feedback packet (v0.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasinFeedback {
    pub basin_id: String,           // unique ID (UUID or hash)
    pub type_: BasinType,           // Valley | Ridge | Peak
    pub coords_2d: [f32; 2],        // projection coordinates for viz

    // Semantic representative (REAL concept, not phantom centroid)
    pub rep_id: String,             // medoid rationale_hash
    pub rep_phrase: String,         // phrase of the medoid

    // Cluster metadata
    pub contributors: Vec<String>,  // rationale_hash list (top-k if large)
    pub nd_cohesion: f32,           // silhouette [-1,1]
    pub nd_radius: f32,             // N-D radius covering p% of members
    pub persistence: u32,           // ticks this basin survived
    pub tempo: Tempo,               // tempo of the basin (from dominant contributors)

    // Optional fields
    pub centroid: Option<Vec<f32>>,         // N-D mean (diagnostic only, never semantic)
    pub endpoints: Option<Vec<String>>,     // ridge: [rep_id_A, rep_id_B]
    pub decompose_into: Option<Vec<String>>, // peak: suggested split medoids
    pub recommended_action: Action,         // what to do with this basin

    // Instant feedback (PreCard template)
    pub precard: Option<PreCard>,           // instant template, no LLM needed

    pub thresholds: Option<ThresholdSnapshot>, // for logging/debugging
    pub timestamp: u64,                     // epoch ms when basin matured
}

/// Ledger entry for N-D vector storage
#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub vector: Vec<f32>,           // 768d embedding
    pub rationale_hash: String,     // unique ID
    pub agent_id: String,
    pub provenance: String,         // context pointer
    pub timestamp: u64,
    pub tempo: Tempo,               // for decay logic
    pub coords_2d: Option<[f32; 2]>, // from projection (computed on demand)
}

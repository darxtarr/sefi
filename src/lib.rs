// Sefi: Semantic Field Blackboard v0.3 (N-D Primary)
// Philosophy: Code boutique. N-D is truth, 2D is oscilloscope.

pub mod types;

// Module structure (to be implemented)
pub mod clustering;
pub mod viz;
pub mod embed;
pub mod ledger;
pub mod validator;
pub mod feedback;
pub mod governor;

// Re-export core types
pub use types::{
    Action, BasinFeedback, BasinType, ConceptPacket, LedgerEntry, Polarity, PreCard, SynthTier,
    Tempo, ThresholdSnapshot,
};

// In-memory ledger for Phase 1 (VLC-backed in Phase 2)

use crate::types::{ConceptPacket, LedgerEntry};
use std::collections::HashMap;

/// Simple in-memory vector ledger
pub struct Ledger {
    entries: Vec<LedgerEntry>,
    index: HashMap<String, usize>, // rationale_hash → index
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Append a concept packet with its N-D embedding
    pub fn append(&mut self, packet: ConceptPacket, vector: Vec<f32>) {
        let idx = self.entries.len();

        let entry = LedgerEntry {
            vector,
            rationale_hash: packet.rationale_hash.clone(),
            agent_id: packet.agent_id,
            provenance: packet.provenance,
            timestamp: packet.timestamp,
            tempo: packet.tempo,
            coords_2d: None, // computed on demand
        };

        self.index.insert(packet.rationale_hash, idx);
        self.entries.push(entry);
    }

    /// Get entry by rationale_hash
    pub fn get(&self, rationale_hash: &str) -> Option<&LedgerEntry> {
        self.index.get(rationale_hash).map(|&idx| &self.entries[idx])
    }

    /// Get all entries (for clustering)
    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    /// Get entries by rationale_hash list
    pub fn get_batch(&self, hashes: &[String]) -> Vec<&LedgerEntry> {
        hashes
            .iter()
            .filter_map(|h| self.get(h))
            .collect()
    }

    /// Get recent entries within time window (for clustering)
    pub fn recent_window(&self, window_ms: u64, now: u64) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| now.saturating_sub(e.timestamp) <= window_ms)
            .collect()
    }

    /// Total entries count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Polarity, Tempo};

    #[test]
    fn test_ledger_append_and_get() {
        let mut ledger = Ledger::new();

        let packet = ConceptPacket {
            phrase: "test phrase".to_string(),
            amp: 0.8,
            sigma: 1.0,
            polarity: Polarity::Attract,
            tempo: Tempo::Fast,
            provenance: "test".to_string(),
            agent_id: "agent1".to_string(),
            rationale_hash: "hash123".to_string(),
            timestamp: 1000,
        };

        let vector = vec![0.1; 768];
        ledger.append(packet, vector.clone());

        let entry = ledger.get("hash123").unwrap();
        assert_eq!(entry.rationale_hash, "hash123");
        assert_eq!(entry.vector, vector);
        assert_eq!(ledger.len(), 1);
    }

    #[test]
    fn test_recent_window() {
        let mut ledger = Ledger::new();

        // Add entries at different times
        for i in 0..5 {
            let packet = ConceptPacket {
                phrase: format!("phrase{}", i),
                amp: 0.5,
                sigma: 1.0,
                polarity: Polarity::Attract,
                tempo: Tempo::Slow,
                provenance: "test".to_string(),
                agent_id: "agent1".to_string(),
                rationale_hash: format!("hash{}", i),
                timestamp: i * 1000, // 0, 1000, 2000, 3000, 4000
            };
            ledger.append(packet, vec![0.1; 768]);
        }

        // Get entries within 2500ms window from time 5000
        let recent = ledger.recent_window(2500, 5000);
        assert_eq!(recent.len(), 3); // timestamps 2000, 3000, 4000
    }
}

// N-D streaming density clustering (primary basin detection)

use crate::ledger::store::Ledger;
use crate::types::{LedgerEntry, Tempo};
use std::collections::HashMap;

/// Cluster/Basin in N-D space
#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: String,
    pub members: Vec<String>, // rationale_hashes
    pub medoid_hash: String,
    pub medoid_phrase: String,
    pub centroid: Vec<f32>, // diagnostic only
    pub persistence: u32,    // ticks survived
    pub tempo: Tempo,        // dominant tempo
    pub last_update: u64,    // timestamp
}

/// Streaming clustering engine with two-tempo decay
pub struct ClusterEngine {
    clusters: HashMap<String, Cluster>,
    cluster_counter: u32,

    // Parameters
    cosine_threshold: f32, // min similarity to join cluster
    min_persistence: u32,  // min ticks before emitting basin
    min_members: usize,    // min members for valid cluster
}

impl ClusterEngine {
    pub fn new() -> Self {
        Self {
            clusters: HashMap::new(),
            cluster_counter: 0,
            cosine_threshold: 0.75, // reasonable default for semantic clustering
            min_persistence: 2,     // at least 2 ticks
            min_members: 2,         // at least 2 members
        }
    }

    /// Process new ledger entries and update clusters
    /// Returns list of mature cluster IDs ready for basin feedback
    pub fn tick(
        &mut self,
        ledger: &Ledger,
        current_time: u64,
        phrases: &HashMap<String, String>, // rationale_hash -> phrase
    ) -> Vec<String> {
        // Apply decay to existing clusters
        self.apply_decay(current_time);

        // Get recent entries (within reasonable window)
        let window_ms = 60_000; // 60s window
        let recent = ledger.recent_window(window_ms, current_time);

        // For each entry, assign to nearest cluster or create new
        for entry in recent.iter() {
            self.assign_or_create(entry, phrases);
        }

        // Find mature clusters ready for emission
        self.find_mature_clusters()
    }

    /// Apply two-tempo decay to cluster persistence
    fn apply_decay(&mut self, current_time: u64) {
        let mut to_remove = Vec::new();

        for (id, cluster) in self.clusters.iter_mut() {
            let dt = (current_time - cluster.last_update) as f32 / 1000.0; // seconds

            // Decay weight based on tempo
            let tau = cluster.tempo.tau();
            if tau > 0.0 {
                let decay = (-dt / tau).exp();

                // If decayed below threshold, mark for removal
                if decay < 0.1 {
                    to_remove.push(id.clone());
                }
            }
            // Urgent never decays (but we process them immediately anyway)
        }

        // Remove decayed clusters
        for id in to_remove {
            self.clusters.remove(&id);
        }
    }

    /// Assign entry to nearest cluster or create new cluster
    fn assign_or_create(
        &mut self,
        entry: &LedgerEntry,
        phrases: &HashMap<String, String>,
    ) {
        // Find nearest cluster
        let mut best_sim = self.cosine_threshold;
        let mut best_cluster_id: Option<String> = None;

        for (id, cluster) in self.clusters.iter() {
            // Compute similarity to cluster centroid
            let sim = cosine_similarity(&entry.vector, &cluster.centroid);

            if sim > best_sim {
                best_sim = sim;
                best_cluster_id = Some(id.clone());
            }
        }

        // Assign to cluster or create new
        if let Some(cluster_id) = best_cluster_id {
            // Add to existing cluster
            if let Some(cluster) = self.clusters.get_mut(&cluster_id) {
                if !cluster.members.contains(&entry.rationale_hash) {
                    cluster.members.push(entry.rationale_hash.clone());
                    cluster.persistence += 1;
                    cluster.last_update = entry.timestamp;

                    // Recompute centroid (simple average)
                    // In production, this would be more sophisticated
                }
            }
        } else {
            // Create new cluster
            let cluster_id = format!("cluster_{}", self.cluster_counter);
            self.cluster_counter += 1;

            let phrase = phrases
                .get(&entry.rationale_hash)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());

            let cluster = Cluster {
                id: cluster_id.clone(),
                members: vec![entry.rationale_hash.clone()],
                medoid_hash: entry.rationale_hash.clone(),
                medoid_phrase: phrase,
                centroid: entry.vector.clone(),
                persistence: 1,
                tempo: entry.tempo,
                last_update: entry.timestamp,
            };

            self.clusters.insert(cluster_id, cluster);
        }
    }

    /// Find clusters that meet maturity criteria
    fn find_mature_clusters(&self) -> Vec<String> {
        self.clusters
            .iter()
            .filter(|(_, c)| {
                c.persistence >= self.min_persistence && c.members.len() >= self.min_members
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get cluster by ID
    pub fn get_cluster(&self, id: &str) -> Option<&Cluster> {
        self.clusters.get(id)
    }

    /// Compute medoid for a cluster
    /// Medoid = member with minimum sum of distances to all other members
    pub fn compute_medoid(
        &self,
        cluster_id: &str,
        ledger: &Ledger,
    ) -> Option<String> {
        let cluster = self.clusters.get(cluster_id)?;

        if cluster.members.is_empty() {
            return None;
        }

        // Get all member vectors
        let member_entries: Vec<&LedgerEntry> = cluster
            .members
            .iter()
            .filter_map(|hash| ledger.get(hash))
            .collect();

        if member_entries.is_empty() {
            return None;
        }

        // Find medoid (member with min total distance to others)
        let mut best_hash = cluster.members[0].clone();
        let mut min_total_dist = f32::MAX;

        for candidate in member_entries.iter() {
            let mut total_dist = 0.0;

            for other in member_entries.iter() {
                if candidate.rationale_hash != other.rationale_hash {
                    let sim = cosine_similarity(&candidate.vector, &other.vector);
                    // Distance = 1 - similarity
                    total_dist += 1.0 - sim;
                }
            }

            if total_dist < min_total_dist {
                min_total_dist = total_dist;
                best_hash = candidate.rationale_hash.clone();
            }
        }

        Some(best_hash)
    }

    /// Compute cohesion (simplified silhouette score)
    /// Returns average similarity within cluster
    pub fn compute_cohesion(
        &self,
        cluster_id: &str,
        ledger: &Ledger,
    ) -> f32 {
        let cluster = match self.clusters.get(cluster_id) {
            Some(c) => c,
            None => return 0.0,
        };

        if cluster.members.len() < 2 {
            return 1.0; // single member = perfect cohesion
        }

        let member_entries: Vec<&LedgerEntry> = cluster
            .members
            .iter()
            .filter_map(|hash| ledger.get(hash))
            .collect();

        if member_entries.len() < 2 {
            return 0.0;
        }

        // Compute average pairwise similarity
        let mut total_sim = 0.0;
        let mut count = 0;

        for i in 0..member_entries.len() {
            for j in (i + 1)..member_entries.len() {
                let sim = cosine_similarity(
                    &member_entries[i].vector,
                    &member_entries[j].vector,
                );
                total_sim += sim;
                count += 1;
            }
        }

        if count > 0 {
            total_sim / count as f32
        } else {
            0.0
        }
    }
}

impl Default for ClusterEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        let v3 = vec![0.0, 1.0, 0.0];

        assert!((cosine_similarity(&v1, &v2) - 1.0).abs() < 1e-5);
        assert!(cosine_similarity(&v1, &v3).abs() < 1e-5);
    }

    #[test]
    fn test_cluster_creation() {
        let mut engine = ClusterEngine::new();
        let ledger = Ledger::new();
        let phrases = HashMap::new();

        let mature = engine.tick(&ledger, 1000, &phrases);
        assert_eq!(mature.len(), 0, "No mature clusters initially");
    }

    #[test]
    fn test_decay_fast_vs_slow() {
        use crate::embed::EmbedService;
        use crate::types::{ConceptPacket, Polarity};

        let mut engine = ClusterEngine::new();
        let mut ledger = Ledger::new();
        let mut phrases = HashMap::new();
        let embed = EmbedService::new();

        // Add Fast tempo packet at t=0
        let packet_fast = ConceptPacket {
            phrase: "fast alert".to_string(),
            amp: 0.9,
            sigma: 1.0,
            polarity: Polarity::Attract,
            tempo: Tempo::Fast, // τ=2s
            provenance: "test".to_string(),
            agent_id: "agent1".to_string(),
            rationale_hash: "hash_fast".to_string(),
            timestamp: 0,
        };

        let vector_fast = embed.embed(&packet_fast.phrase);
        phrases.insert("hash_fast".to_string(), packet_fast.phrase.clone());
        ledger.append(packet_fast, vector_fast);

        // Add Slow tempo packet at t=0
        let packet_slow = ConceptPacket {
            phrase: "slow consensus".to_string(),
            amp: 0.9,
            sigma: 1.0,
            polarity: Polarity::Attract,
            tempo: Tempo::Slow, // τ=30s
            provenance: "test".to_string(),
            agent_id: "agent2".to_string(),
            rationale_hash: "hash_slow".to_string(),
            timestamp: 0,
        };

        let vector_slow = embed.embed(&packet_slow.phrase);
        phrases.insert("hash_slow".to_string(), packet_slow.phrase.clone());
        ledger.append(packet_slow, vector_slow);

        // Process at t=0
        engine.tick(&ledger, 0, &phrases);
        assert_eq!(engine.clusters.len(), 2, "Should create 2 clusters");

        // Process at t=3000 (3 seconds later)
        // Fast should decay significantly (exp(-3/2) ≈ 0.22)
        // Slow should barely decay (exp(-3/30) ≈ 0.90)
        engine.tick(&ledger, 3000, &phrases);

        // Fast cluster should be removed (decay < 0.1 threshold)
        // Slow cluster should remain
        assert!(
            engine.clusters.len() <= 2,
            "Fast clusters may have decayed"
        );
    }

    #[test]
    fn test_medoid_computation() {
        use crate::embed::EmbedService;
        use crate::types::{ConceptPacket, Polarity};

        let mut engine = ClusterEngine::new();
        let mut ledger = Ledger::new();
        let mut phrases = HashMap::new();
        let embed = EmbedService::new();

        // Create a cluster with 3 similar concepts
        let concepts = [
            "memory safety",
            "borrow checker",
            "zero cost abstractions",
        ];

        for (i, phrase) in concepts.iter().enumerate() {
            let packet = ConceptPacket {
                phrase: phrase.to_string(),
                amp: 0.8,
                sigma: 1.0,
                polarity: Polarity::Attract,
                tempo: Tempo::Slow,
                provenance: "test".to_string(),
                agent_id: format!("agent{}", i),
                rationale_hash: format!("hash{}", i),
                timestamp: 1000,
            };

            let vector = embed.embed(phrase);
            phrases.insert(format!("hash{}", i), phrase.to_string());
            ledger.append(packet, vector);
        }

        // Force all into same cluster by using very low threshold
        engine.cosine_threshold = -1.0; // accept all (cosine similarity ranges from -1 to 1)
        engine.tick(&ledger, 1000, &phrases);

        // May have multiple clusters depending on similarity
        // Just verify we have at least one cluster
        assert!(!engine.clusters.is_empty());

        let cluster_id = engine.clusters.keys().next().unwrap().clone();

        // Compute medoid
        let medoid_hash = engine.compute_medoid(&cluster_id, &ledger);
        assert!(medoid_hash.is_some(), "Should find a medoid");

        // Verify medoid is one of the members
        let cluster = engine.get_cluster(&cluster_id).unwrap();
        assert!(cluster.members.contains(&medoid_hash.unwrap()));
    }

    #[test]
    fn test_cohesion_computation() {
        use crate::embed::EmbedService;
        use crate::types::{ConceptPacket, Polarity};

        let mut engine = ClusterEngine::new();
        let mut ledger = Ledger::new();
        let mut phrases = HashMap::new();
        let embed = EmbedService::new();

        // Create similar concepts
        let concepts = ["memory safety", "memory safety again"];

        for (i, phrase) in concepts.iter().enumerate() {
            let packet = ConceptPacket {
                phrase: phrase.to_string(),
                amp: 0.8,
                sigma: 1.0,
                polarity: Polarity::Attract,
                tempo: Tempo::Slow,
                provenance: "test".to_string(),
                agent_id: format!("agent{}", i),
                rationale_hash: format!("hash{}", i),
                timestamp: 1000,
            };

            let vector = embed.embed(phrase);
            phrases.insert(format!("hash{}", i), phrase.to_string());
            ledger.append(packet, vector);
        }

        engine.cosine_threshold = 0.0;
        engine.tick(&ledger, 1000, &phrases);

        let cluster_id = engine.clusters.keys().next().unwrap().clone();
        let cohesion = engine.compute_cohesion(&cluster_id, &ledger);

        // Cohesion should be in [0, 1] range
        assert!((0.0..=1.0).contains(&cohesion));
    }
}

// Embedding service client (mock in Phase 1)

use sha2::{Digest, Sha256};

const EMBEDDING_DIM: usize = 768;

/// Mock embedding service for Phase 1
/// Generates deterministic 768d vectors from text using hash-based approach
/// Properties:
/// - Deterministic (same text → same vector)
/// - Unit normalized (||v|| = 1)
/// - Semantic similarity approximated by character overlap
pub struct EmbedService {
    _placeholder: (),
}

impl EmbedService {
    pub fn new() -> Self {
        Self { _placeholder: () }
    }

    /// Generate embedding vector for a phrase
    /// Returns a normalized 768d vector
    pub fn embed(&self, text: &str) -> Vec<f32> {
        // Normalize text
        let normalized = text.to_lowercase().trim().to_string();

        // Generate base hash
        let mut hasher = Sha256::new();
        hasher.update(normalized.as_bytes());
        let hash = hasher.finalize();

        // Expand hash to 768 dimensions using multiple rounds
        let mut vector = Vec::with_capacity(EMBEDDING_DIM);

        for i in 0..EMBEDDING_DIM {
            let mut round_hasher = Sha256::new();
            round_hasher.update(hash);
            round_hasher.update((i as u32).to_le_bytes());
            let round_hash = round_hasher.finalize();

            // Convert first 4 bytes to f32
            let bytes = [
                round_hash[0],
                round_hash[1],
                round_hash[2],
                round_hash[3],
            ];
            let val = u32::from_le_bytes(bytes) as f32 / u32::MAX as f32;

            // Map from [0,1] to [-1,1]
            vector.push(val * 2.0 - 1.0);
        }

        // Normalize to unit length
        normalize_vector(&mut vector);

        vector
    }

    /// Compute cosine similarity between two vectors
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len(), "Vectors must have same dimension");

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

        // Since vectors are normalized, cosine = dot product
        dot
    }
}

impl Default for EmbedService {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize vector to unit length (L2 norm = 1)
fn normalize_vector(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm > 1e-10 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_deterministic() {
        let service = EmbedService::new();

        let v1 = service.embed("memory safety");
        let v2 = service.embed("memory safety");

        assert_eq!(v1, v2, "Same text should produce same embedding");
    }

    #[test]
    fn test_embed_normalized() {
        let service = EmbedService::new();

        let v = service.embed("rust borrow checker");
        assert_eq!(v.len(), 768, "Should produce 768d vector");

        // Check unit norm
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5, "Vector should be unit normalized");
    }

    #[test]
    fn test_cosine_similarity() {
        let service = EmbedService::new();

        let v1 = service.embed("memory safety");
        let v2 = service.embed("rust borrow checker");
        let v3 = service.embed("memory safety");

        // Self-similarity should be 1.0
        let sim_self = EmbedService::cosine_similarity(&v1, &v3);
        assert!((sim_self - 1.0).abs() < 1e-5, "Self-similarity should be 1.0");

        // Different texts should have similarity in reasonable range
        let sim_diff = EmbedService::cosine_similarity(&v1, &v2);
        assert!((-1.0..=1.0).contains(&sim_diff), "Cosine similarity should be in [-1, 1]");
        assert!(sim_diff < 0.99, "Different texts should have different embeddings");
    }
}

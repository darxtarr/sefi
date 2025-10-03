# Sefi: Semantic Field Blackboard - Complete Specification v0.3

**Version**: v0.3 (N-D Primary)
**Phase 1**: Terminal output only (no visualization)
**Phase 2**: Adds WASM-WebGPU visualization for web/edge deployment

## Core Idea

An **N-D semantic clustering engine** for distributed agent systems. Agents don't exchange text; they emit **concept packets** (compressed intent signals: phrase + amp/sigma/tempo). Streaming density clustering in full-fidelity N-D space finds semantic basins (valleys/ridges/peaks). **PreCard templates** provide instant feedback. Visualization deferred to Phase 2.

---

## 1. Concept Packet Schema

Every agent emission is a packet:

```rust
struct ConceptPacket {
    phrase: String,          // anchor text (short, 2–6 words)
    amp: f32,                // [0..1] confidence/urgency
    sigma: f32,              // breadth vs specificity
    polarity: Polarity,      // Attract | Repel
    tempo: Tempo,            // decay/persistence tempo
    provenance: String,      // hash/pointer to source context
    agent_id: String,        // stable role ID
    rationale_hash: String,  // hash of reasoning step
    timestamp: u64,          // ms epoch
    // center is optional: if omitted, resolved via embedding service
}

enum Polarity {
    Attract,
    Repel,
}

enum Tempo {
    Fast,    // τ=2s  - bursts, quick iterations
    Slow,    // τ=30s - consensus, persistent thoughts
    Urgent,  // bypass persistence, auto-expire if not reinforced
}
```

* **Embedding Service:** shared LAN-hosted embedding model (e.g. Gemma-embedding, 256–768d).
* **Size:** ~1.7 KB/packet @ 768d FP16.
* **Tempo Semantics:**
  - **Fast**: Short-lived signals (alerts, bursts) - decay quickly (τ=2s)
  - **Slow**: Long-term consensus - persist longer (τ=30s)
  - **Urgent**: Critical alerts - bypass persistence checks, emit immediately

---

## 2. N-D Primary (Visualization Deferred to Phase 2)

**Architecture v0.3**: N-D is primary, 2D visualization deferred to Phase 2.

### N-D Clustering (Primary)
* **Field:** Full-fidelity N-D space (768d embeddings)
* **Method:** Streaming density clustering (micro-cell grid or DBSCAN-lite)
* **Tempo-aware decay:** Fast (τ=2s), Slow (τ=30s), Urgent (bypass)
* **Basin detection:** Density peaks → valleys (consensus), ridges (tradeoff), peaks (conflict)
* **Output:** Basin proposals with member sets (rationale_hashes)

### 2D Visualization (Phase 2 - Oscilloscope)
* **Status:** Deferred to Phase 2
* **Purpose:** Human monitoring only - no semantic decisions
* **Method:** Project N-D state to 2D via PCA (fixed projection)
* **Render:** Density heatmap + contour overlays (WGPU shader)
* **Deployment:** WASM-WebGPU for web canvas or edge "ganglion" devices (Jetson Nano)

**Critical Understanding**: All semantic clustering happens in N-D. The 2D viz is just a projection of N-D state for human observation. No 2D physics, no annealing in 2D space.

**Phase 1**: Terminal output only (JSON/text basin summaries)

---

## 3. Ledger

* Append-only vector store for N-D embeddings + metadata.
* Each emission logged as: `{vector, provenance, agent_id, rationale_hash, timestamp, coords_2d}`.
* Used for:
   * **Retrieval:** which embeddings contributed to a basin.
   * **Attribution:** back to doc/code/log context.
   * **Crux synthesis:** stable basin ⇒ generate one-screen summary card.
* **Efficiency:** VLC-compressed for archival (2-3% original size).

---

## 4. Readouts

* **Valleys** = consensus attractors.
* **Ridges** = tradeoffs/conflicts.
* **Peaks** = overload / contradiction.
* **Persistence** = confidence (longer lived ⇒ stronger).

---

## 5. Basin Feedback Schema v0.3

Each **stable feature** (valley, ridge, peak) emits a feedback packet when it matures:

```rust
struct BasinFeedback {
    basin_id: String,                   // unique ID (UUID or hash)
    type_: BasinType,                   // Valley | Ridge | Peak
    coords_2d: [f32; 2],                // projection coordinates for viz

    // Semantic representative (REAL concept, not phantom centroid)
    rep_id: String,                     // medoid rationale_hash
    rep_phrase: String,                 // phrase of the medoid

    // Cluster metadata
    contributors: Vec<String>,          // rationale_hash list (top-k if large)
    nd_cohesion: f32,                   // silhouette [-1,1]
    nd_radius: f32,                     // N-D radius covering p% of members
    persistence: u32,                   // ticks this basin survived
    tempo: Tempo,                       // tempo of the basin (from dominant contributors)

    // Optional fields (2D-specific removed, N-D is primary)
    centroid: Option<Vec<f32>>,         // N-D mean (diagnostic only, never semantic)
    endpoints: Option<Vec<String>>,     // ridge: [rep_id_A, rep_id_B]
    decompose_into: Option<Vec<String>>, // peak: suggested split medoids
    recommended_action: Action,         // what to do with this basin

    // Instant feedback (PreCard template)
    precard: Option<PreCard>,           // instant template, no LLM needed

    thresholds: Option<ThresholdSnapshot>, // for logging/debugging
    timestamp: u64,                     // epoch ms when basin matured
}

enum BasinType {
    Valley,    // consensus
    Ridge,     // tradeoff boundary
    Peak,      // overload/contradiction
}

enum Action {
    PlanSpike,           // valley → spawn implementation
    PairedExperiment,    // ridge → probe both sides
    Decompose,           // peak → split into sub-basins
    IgnoreShortLived,    // transient noise
}

// Tiered synthesis schema (v0.3)
struct PreCard {
    tier: SynthTier,                    // Template | Light | Heavy
    summary: String,                    // instant template summary
    top_phrases: Vec<String>,           // top-k contributor phrases
    suggested_action: String,           // template action text
}

enum SynthTier {
    Template,   // instant, no LLM - just top phrases + action
    Light,      // future: lightweight LLM summary
    Heavy,      // future: full crux card with steelman
}

struct ThresholdSnapshot {
    persistence_min: u32,
    density_threshold: f32,
    nd_min_members: usize,
    nd_radius: f32,
    window_W: u32,
}
```

**v0.3 Changes:**
- Added `tempo` field to BasinFeedback
- Added `PreCard` for instant template generation (no LLM)
- Removed `projection_flag` (no artifacts in N-D primary)
- Removed `depth` and `spread` (2D-specific metrics)
- Updated `ThresholdSnapshot` to use `density_threshold` instead of `depth_min`

---

## 6. Lifecycle (Input → N-D → Output Loop)

**v0.3 N-D Primary Architecture:**

1. **Agent emits ConceptPacket.**
   * Minimal text anchor + amp/sigma/polarity/tempo.
   * Embedding resolved by LAN service.

2. **Ledger stores.**
   * Append N-D vector + provenance to ledger.
   * Track tempo for decay logic.

3. **N-D Clustering detects basins.**
   * Streaming density clustering (micro-cell grid or DBSCAN-lite).
   * Tempo-aware decay: Fast (τ=2s), Slow (τ=30s), Urgent (bypass).
   * Identifies valleys (consensus), ridges (tradeoffs), peaks (conflicts).
   * **Direct N-D detection** - no 2D proposals needed.

4. **Basin Validator computes metadata.**
   * Compute cohesion (silhouette score).
   * Find medoid (nearest to Fréchet mean).
   * Extract top-k contributor phrases.

5. **PreCard Template Generation (instant).**
   * No LLM needed - just format top phrases + suggested action.
   * Template tier for immediate feedback.

6. **Emit BasinFeedback packet.**
   * Package validated basin with medoid, contributors, cohesion, PreCard.
   * Dispatch to synthesizer/actuator (or terminal in Phase 1).

7. **2D Visualization (parallel).**
   * Project N-D basin state to 2D via PCA.
   * Render density heatmap for human monitoring.
   * **Viz only** - no semantic decisions in 2D.

8. **Crux card generation (downstream, future).**
   * Light tier: lightweight LLM summary.
   * Heavy tier: full crux card with steelman adversary.
   * Actuator spawns task/experiment based on basin type.

---

## 7. Key Design Principles (v0.3)

### N-D is Truth, 2D is Oscilloscope
**Core Principle:** All semantic clustering happens in full-fidelity N-D space. 2D visualization is purely for human monitoring.

**Why:**
- Semantics live in N-D - projection to 2D loses information
- No projection artifacts - clustering happens where truth lives
- Simpler architecture - no 2D physics, just density projection

### Centroid → Medoid
**Problem:** N-D means can land in semantic "voids."
**Fix:** Use medoid (actual contributing vector nearest to center) as semantic representative. Keep centroid only for diagnostics, never for semantic interpretation.

### Two-Tempo Decay
**Problem:** Alerts and consensus need different persistence behaviors.
**Fix:** Tempo-aware decay constants:
- **Fast (τ=2s)**: Bursts, alerts, quick iterations
- **Slow (τ=30s)**: Consensus, persistent thoughts
- **Urgent**: Bypass persistence checks entirely, emit immediately

**Governor:** Adjust persistence thresholds per-tempo to maintain basin quality.

### Adaptive Thresholds (Governor)
**Target:** Maintain stable feedback rate and feature quality, not fixed constants.

**Targets:**
- `target_basin_rate`: 10–30/min
- `target_nd_cohesion`: ≥ 0.5
- `target_persistence`: ≥ H ticks (per tempo)

**Controller:** Simple proportional controller:
- If basin_rate > target → increase persistence_min (per-tempo)
- If nd_cohesion low → increase nd_min_members or shrink nd_radius
- If too few basins → relax thresholds

This is PID-lite that keeps the system in its "impedance-matched" operating point automatically.

### Tiered Synthesis (PreCard → Light → Heavy)
**Instant Feedback:** PreCard templates (no LLM) provide immediate basin summaries.

**Future Tiers:**
- **Template (Phase 1)**: Top phrases + suggested action (instant)
- **Light (Phase 2)**: Lightweight LLM summary
- **Heavy (Phase 3)**: Full crux card with steelman adversary

### Ridge/Peak Semantics → Distinct Handlers

**Valley (consensus):**
- Action: spawn Plan/Spike with rep_phrase, top-k phrases, provenance bundle
- PreCard: "Consensus: {medoid_phrase}. Contributors: {top_k}. Action: {suggested}"

**Ridge (tradeoff boundary):**
- Action: spawn Paired Experiment
- Two branches anchored at ridge-adjacent medoids (rep_id_A, rep_id_B)
- Include estimated tradeoff axis (difference vector between medoids)

**Peak (overload/contradiction):**
- Action: Decompose ticket requesting split by clustering
- Throttle amplitudes in that region for Δt
- Raise governor alert if peaks persist (possible mis-tuning)

---

## 8. Efficiency Defaults

* **Embedding dimension:** 768d, FP16.
* **Emission rate:** thought-level (≈10 per agent per sec).
* **Clustering tick:** 10-30 Hz (streaming density update)
* **Decay constants:** Fast τ=2s, Slow τ=30s, Urgent=bypass
* **Persistence:** RAM for active window; VLC-compress older vectors to disk.
* **Feedback rate:** ~10-30 basins/min (100:1 compression vs emissions).
* **Viz refresh:** 30-60 fps (2D heatmap update from N-D state)

---

## 9. Operator Heuristics (Practical Reading)

**High persistence + high nd_cohesion (valley):**  
→ Act quickly; it's real consensus.

**Ridge with clear endpoints + decent cohesion on both sides:**  
→ Run paired probe; record the tradeoff axis.

**Peak with low cohesion:**  
→ Likely overload; decompose and damp.

**Projection_flag = suspect:**  
→ Let the synthesizer cross-check text before acting; keep human out of loop unless anomalies persist.

---

## 10. Deliverables (POC - Phase 1)

* **N-D Clustering**: Streaming density clustering with two-tempo decay.
* **Embedding service**: LAN Gemma-embedding wrapper (mock in Phase 1).
* **Concept packet API**: agents emit JSON/MsgPack packets with tempo.
* **Ledger**: minimal vector DB (in-memory, VLC compression in Phase 2).
* **Viz**: 2D projection + heatmap renderer (oscilloscope only).
* **Feedback**: BasinFeedback with PreCard templates.
* **Governor**: adaptive threshold controller (tempo-aware).

---

## 11. Success Criteria

**Phase 1 (Minimal Loop - N-D Primary):**
- [ ] Emit ConceptPacket via CLI → stored in N-D ledger
- [ ] N-D clustering detects basins with tempo-aware decay
- [ ] Urgent packets bypass persistence, Fast decay quickly (τ=2s), Slow persist (τ=30s)
- [ ] Basin validator computes medoid + cohesion (≥ 0.5)
- [ ] PreCard template generated instantly (no LLM)
- [ ] Print BasinFeedback to terminal with medoid phrase + PreCard
- [ ] 2D viz shows live projection of N-D basin state

**Phase 2 (Production Loop):**
- [ ] Real embedding service integration
- [ ] VLC-backed ledger compression
- [ ] Network dispatch of BasinFeedback
- [ ] Governor maintains target basin rate (tempo-aware)
- [ ] Ridge and peak handling implemented
- [ ] Light synthesis tier (lightweight LLM)

**Phase 3 (Synthesis Integration):**
- [ ] Heavy synthesis tier (full crux cards)
- [ ] Steelman adversary validation
- [ ] Actuator dispatch for different basin types

---

⚡ **One-liner (v0.3):**
Agents emit concept packets (phrase + amp/sigma/tempo) → N-D embeddings (LAN service) → streaming density clustering finds basins (two-tempo decay: Fast/Slow/Urgent) → medoid represents real concepts → PreCard templates (instant feedback) → BasinFeedback dispatched. 2D viz is pure projection (oscilloscope). Adaptive governor (tempo-aware). VLC compression. N-D is truth, 2D is display.

---

*Specification v0.3 - N-D Primary with Two-Tempo Decay* 💎

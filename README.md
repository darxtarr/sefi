# Sefi: Semantic Field Blackboard

**Version**: v0.3 (N-D Primary)
**Status**: 🏗️ Phase 1 - Foundation Complete, Ready to Build
**Philosophy**: Code boutique. N-D is truth, 2D is oscilloscope.

---

## What Is Sefi?

Sefi is a **real-time consensus filter for agent swarms**.

Instead of dozens of LLMs spitting paragraphs, Sefi lets them emit **concept packets** — tiny semantic nudges — into a shared space. Sefi continuously filters these signals, extracting the few stable patterns that persist and matter.

Think of it as an **oscilloscope for collective reasoning**: noise blurs away, only persistent structures survive. You glance at the screen and see not a wall of text, but 2–3 "live basins" of agreement, tradeoff, or anomaly.

### Why It Exists

Modern toolchains drown you in alerts, logs, and AI outputs. Humans can't read it all.

Sefi acts as a **governor**: it digests thousands of micro-signals per minute and outputs actionable summaries at human pace.

### Example Use Cases

- **DevOps**: Dozens of diagnostic agents process logs → Sefi produces one Crux Card: "GPU allocation bottleneck."
- **Code Review**: Agents highlight issues → Sefi clusters them into a single "Refactor module Y" card.
- **Incident Routing**: Alerts flood in → Sefi filters to "API 502 EU region" anomaly.

### How It Works

1. **Agents emit ConceptPackets**: Short anchor phrase + confidence (amp) + specificity (sigma) + tempo + provenance.
2. **Shared embedding model**: Maps phrases into a common 768-dimensional space.
3. **N-D Governor**: Streaming density estimator with two-tempo decay clusters signals in real time.
4. **Medoids, not centroids**: Clusters are anchored to real concepts, not ghosts.
5. **Fast path for urgent spikes**: High-amp packets with risk tag trigger immediate alerts.
6. **BasinFeedback**: When a cluster matures (≥T ticks, ≥m members), Sefi emits a feedback packet with medoid phrase, contributors, cohesion, and recommended action.
7. **Tiered Synthesis**: Cheap PreCards are emitted instantly; heavy LLM synthesis (Crux Cards) only for priority basins.
8. **2-D Canvas** (Phase 2): A monitor only — live heatmap for humans, no decisions made here.

### What You Get

- **Noise → structure**: Thousands of agent emissions reduced to a handful of stable signals.
- **Human-rate output**: Actionable Crux Cards you can read or feed directly into tickets/alerts.
- **Auditability**: Every cluster tied back to actual agent outputs via provenance hashes.

---

## Key Concepts (v0.3)

### Concept Packet
Minimal emission from an agent:
- **phrase**: 2-6 word anchor text
- **amp**: confidence/urgency [0..1]
- **sigma**: breadth vs specificity
- **tempo**: Fast (τ=2s) | Slow (τ=30s) | Urgent (bypass)
- **polarity**: attract | repel
- **metadata**: agent_id, provenance, rationale_hash

### N-D Clustering (Primary)
- **Full-fidelity N-D space** (768d embeddings)
- Streaming density clustering with HNSW + approximate DBSCAN
- Two-tempo decay for different signal types
- Output: clusters (consensus), ridges (tradeoffs), anomalies (outliers)

### Basin Feedback with PreCard
When a cluster matures:
- Emit **BasinFeedback** packet (type, medoid, cohesion, contributors)
- **PreCard template** (instant, no LLM): top phrases + suggested action
- Future tiers: Light/Heavy synthesis with LLM (Phase 2/3)

### Ledger
- Append-only vector store (N-D embeddings + metadata)
- Used for cluster validation and provenance retrieval
- Future: VLC compression for archival efficiency (Phase 2)

### Visualization (Phase 2)
- **WASM-WebGPU target** for web canvas or edge "ganglion" devices
- 2D projection (PCA/UMAP) of N-D state - **monitor only**, no decisions
- Deployable to Jetson Nano or similar edge nodes

---

## Architecture Principle

**N-D is truth, 2D is oscilloscope:**
- All semantic clustering happens in full-fidelity N-D space
- No 2D physics (no annealing, no diffusion in 2D)
- Visualization is pure projection for human monitoring
- Prevents projection artifacts from creating phantom basins

---

## Tech Stack

- **Rust** (boutique code, minimal deps)
- **No GPU needed for Phase 1** (clustering is CPU, viz deferred to Phase 2)
- Core crates: serde, nalgebra, chrono, hnsw (approximate nearest neighbors)
- **No frameworks** (surgical implementations)

**Phase 2 Viz**:
- WGPU for rendering (WASM-WebGPU target)
- Deployable to web canvas or edge devices (Jetson Nano, browser)

---

## Quick Start (Phase 1)

```bash
# Build
cargo build --release

# Emit concept packets
./target/release/sefi emit "memory safety" --amp 0.9 --tempo slow
./target/release/sefi emit "rust borrow checker" --amp 0.8 --tempo slow
./target/release/sefi emit "zero cost abstractions" --amp 0.7 --tempo slow

# Check status (will show basins when clustering is implemented)
./target/release/sefi status
```

**Expected Output** (when Phase 1 complete):
```
Basin detected: Valley
  Medoid: "memory safety"
  Contributors: 3 concepts
  Cohesion: 0.87
  PreCard: "Consensus on memory safety. Top phrases: memory safety,
           rust borrow checker, zero cost abstractions.
           Suggested action: Plan spike on Rust safety guarantees."
```

---

## Documentation

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Complete system architecture (v0.3)
- **[SPEC.md](./docs/SPEC.md)** - Full specification with schemas
- **[STATUS.md](./STATUS.md)** - Current build status
- **[BUILD_PLAN.md](./BUILD_PLAN.md)** - Detailed milestones (coming)

---

## Project Status

**Foundation** (✅ Complete):
- [x] types.rs - Core data structures (Tempo, PreCard, BasinFeedback)
- [x] ledger/store.rs - In-memory ledger with tests
- [x] CLI stub - Command parsing works

**Phase 1 Milestones** (🏗️ To Build):
- [ ] M1.1: Real embeddings + HNSW index + naive cosine grouping + two-tempo decay
- [ ] M1.2: Upgrade to streaming DBSCAN-lite + basin validator (medoid + cohesion)
- [ ] M1.3: Tiered synthesis (PreCard → Crux Card pipeline)
- [ ] M1.4: Wire CLI to engine
- [ ] M1.5: Integration tests + demo script

**Phase 2** (Future):
- Visualization (WASM-WebGPU for web/edge deployment)
- VLC compression for ledger (2-3% compression ratio)
- Network dispatch
- Governor (adaptive thresholds)

**Phase 3** (Future):
- Light/Heavy synthesis tiers (LLM integration)
- Steelman adversary
- Actuator dispatch

---

## Integration with VLC

Sefi will integrate [VLC (Vector-Lattice Compression)](../vlc/README.md) for efficient vector storage in Phase 2:
- **Compression**: Ledger entries compressed to 2-3% of original size
- **Retrieval**: Sub-millisecond query latency (~4700 queries/second)
- **Status**: VLC is production-ready and tested
- **Shared Philosophy**: Boutique code, minimal deps, understand everything

---

## Deployment Vision

**Ganglion Architecture** (Phase 2+):
- Core N-D clustering on server/coordinator
- WASM-WebGPU viz on edge "ganglion" nodes (Jetson Nano, etc.)
- Distributed monitoring, centralized semantic clustering
- Agents emit to local ganglion, federate to core

---

**Ready to Build**: Foundation complete, M1.1 next up
**Philosophy**: Think twice, code once. N-D is truth. 💎

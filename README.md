# Sefi: Semantic Field Blackboard

**Version**: v0.3 (N-D Primary)
**Status**: 🏗️ Phase 1 - Foundation Complete, Ready to Build
**Philosophy**: Code boutique. N-D is truth, 2D is oscilloscope.

---

## What Is Sefi?

Sefi is an **N-D semantic clustering engine** where agents emit concept packets that cluster into persistent basins (consensus/tradeoffs/conflicts). It's designed for distributed agent systems to discover emergent patterns without central coordination.

**Core Innovation**: Agents emit compressed intent signals (phrase + amp/sigma/tempo) → N-D embeddings → streaming density clustering finds basins → instant PreCard templates provide feedback. No text exchange, just semantic field dynamics.

---

## Key Concepts (v0.3)

### Concept Packet
Minimal emission from an agent:
- `phrase`: 2-6 word anchor text
- `amp`: confidence/urgency [0..1]
- `sigma`: breadth vs specificity
- `tempo`: Fast (τ=2s) | Slow (τ=30s) | Urgent (bypass)
- `polarity`: attract | repel
- Metadata: agent_id, provenance, rationale_hash

### N-D Clustering (Primary)
- **Full-fidelity N-D space** (768d embeddings)
- Streaming density clustering (micro-cell grid)
- Two-tempo decay for different signal types
- Output: valleys (consensus), ridges (tradeoffs), peaks (conflicts)

### Basin Feedback with PreCard
When a basin matures:
- Emit BasinFeedback packet (type, medoid, cohesion, contributors)
- **PreCard template** (instant, no LLM): top phrases + suggested action
- Future: Light/Heavy synthesis tiers (LLM-based)

### Ledger
- Append-only vector store (N-D embeddings + metadata)
- VLC-compressed for archival efficiency (Phase 2)
- Used for basin validation and provenance retrieval

### Visualization (Phase 2)
- **WASM-WebGPU target** for web canvas or edge "ganglion" devices
- 2D projection (PCA) of N-D state - **monitor only**, no decisions
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
- **~6-7 crates** (serde, nalgebra, chrono, etc.)
- **No frameworks** (surgical implementations)

**Phase 2 Viz**:
- WGPU for rendering (WASM-WebGPU target)
- Deployable to web canvas or edge devices

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
- [ ] M1.1: Streaming density clustering + two-tempo decay
- [ ] M1.2: Basin validator (medoid + cohesion)
- [ ] M1.3: Feedback emitter (PreCard templates)
- [ ] M1.4: Wire CLI to engine
- [ ] M1.5: Integration tests + demo script

**Phase 2** (Future):
- Visualization (WASM-WebGPU for web/edge deployment)
- Real embedding service
- VLC compression
- Network dispatch
- Governor (adaptive thresholds)

**Phase 3** (Future):
- Light/Heavy synthesis (LLM integration)
- Steelman adversary
- Actuator dispatch

---

## Integration with VLC

Sefi will use [VLC](../vlc/README.md) for efficient vector storage (Phase 2):
- **Compression**: Ledger entries → VLC at 2-3% original size
- **Retrieval**: Pull contributors for basin synthesis (sub-ms latency)
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

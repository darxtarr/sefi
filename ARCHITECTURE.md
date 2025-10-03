# Sefi: Semantic Field Blackboard - Architecture v0.3 (N-D Primary)

**Philosophy**: Code boutique. Surgical recreation. Think twice, code once.

---

## Architecture Decision: N-D is Truth, 2D is Oscilloscope

**The Core Principle:**
- **N-D space is primary**: All semantic clustering, basin detection, and validation happen in full-fidelity N-D
- **2D is pure visualization**: Projection of N-D state for human observation (oscilloscope screen)
- **No 2D physics**: No annealing, no diffusion in 2D. Just live projection of N-D density.

**Rationale**: Semantics live in N-D. Running expensive 2D physics creates projection artifacts and solves the wrong problem. Keep it simple: stream N-D density → find basins → project to 2D for viz.

---

## System Overview

Sefi is an **N-D semantic field** with streaming density clustering. Agents emit concept packets → N-D embeddings → density clustering finds basins → 2D projection visualizes for humans.

**Core Loop**:
```
ConceptPacket → Embedding → N-D Ledger → Streaming Density Clustering → BasinFeedback → Synthesizer → Action
                                    ↓
                              2D Projection (viz only)
```

**2D Role**: Pure visualization. Project N-D basin state to 2D heatmap/contours for human monitoring. No decisions made in 2D space.

---

## Component Map

### 1. N-D Density Clustering (Primary basin detection)
**Purpose**: Streaming density-based clustering in N-D space to find semantic basins.

**Tech**:
- Pure Rust (no GPU needed for this - CPU is fine for clustering)
- Streaming DBSCAN-lite or micro-cell density grid
- Two-tempo decay: Fast (τ=2s), Slow (τ=30s), Urgent (bypass)
- Temporal windowing for persistence filtering

**Inputs**: ConceptPacket → N-D embedding (from ledger)

**Outputs**: Basin proposals (valleys/ridges/peaks) with member sets

**Key Files**:
```
src/clustering/
  density.rs     # Streaming density clustering
  basins.rs      # Basin detection and tracking
  tempo.rs       # Two-tempo decay logic
```

---

### 2. 2D Visualization (Oscilloscope) - **PHASE 2**
**Purpose**: Project N-D state to 2D for human monitoring. Viz only, no physics.

**Status**: Deferred to Phase 2. Phase 1 uses terminal output only.

**Planned Tech**:
- WGPU for rendering (compute shader for density projection)
- Fixed PCA projection for visual stability
- **WASM-WebGPU target** for deployment to web canvas or edge devices ("ganglion" nodes)

**Deployment Options**:
1. **Web Canvas**: Serve via HTTP, render in browser with WebGPU
2. **Native Window**: Local winit + WGPU window
3. **Edge Device**: WASM on Jetson Nano or similar "ganglion" nodes
4. **Headless**: Render to PNG files for monitoring

**Key Files** (Phase 2):
```
src/viz/
  projection.rs  # PCA projection (N-D → 2D)
  renderer.wgsl  # Density heatmap shader
  display.rs     # WebGPU canvas or window management
  server.rs      # Optional web server (axum/warp)
```

**WASM-WebGPU Note**: Sefi's viz layer is designed to compile to WASM for deployment on edge "ganglion" devices. The core N-D clustering runs on server, viz can run distributed on monitoring nodes.

---

### 3. Embedding Bridge
**Purpose**: Resolve concept phrases to N-D vectors via LAN service.

**Tech**: 
- Rust HTTP client (hyper or reqwest, minimal)
- Async batch requests to embedding service
- Cache recent embeddings (LRU, in-memory)

**Interface**:
```rust
async fn embed_batch(phrases: &[String]) -> Result<Vec<Vec<f32>>>
```

**Key Files**:
```
src/embed/
  client.rs      # HTTP client to LAN embedding service
  cache.rs       # LRU cache for recent phrases
```

**Embedding Service** (separate, outside sefi):
- FastAPI + sentence-transformers (Gemma-embedding or similar)
- Single endpoint: POST /embed with JSON array of strings
- Returns: JSON array of float arrays (768d)
- Runs on LAN, sefi clients point to http://embedding-host:8000

---

### 4. Ledger (Vector storage with provenance)
**Purpose**: Append-only store for N-D embeddings + metadata. Used for N-D validation and retrieval.

**Tech**:
- **VLC integration**: Use VLC's compression for archival storage
- Memory-mapped active window (last ~10K packets)
- Batch compress to VLC when window slides

**Schema per entry**:
```rust
struct LedgerEntry {
    vector: Vec<f32>,           // 768d embedding
    rationale_hash: String,     // unique ID
    agent_id: String,
    provenance: String,         // context pointer
    timestamp: u64,
    coords_2d: [f32; 2],        // from projection
}
```

**Key Files**:
```
src/ledger/
  store.rs       # Append, query by ID, batch compress
  window.rs      # Active memory window
  vlc_backend.rs # VLC compression integration
```

---

### 5. Basin Validator
**Purpose**: Compute medoid, cohesion, and metadata for detected N-D basins.

**Tech**:
- Pure Rust, no ML deps (just distance math)
- Silhouette coefficient for cohesion
- Medoid computation (Fréchet mean + nearest neighbor)
- No projection artifacts to worry about (N-D is source of truth)

**Interface**:
```rust
fn validate_basin(
    members: Vec<RationaleHash>,  // from N-D clustering
    ledger: &Ledger
) -> ValidatedBasin
```

**Key Files**:
```
src/validator/
  ndspace.rs     # N-D distance/cohesion metrics
  medoid.rs      # Medoid computation
```

---

### 6. Feedback Emitter (with PreCard templates)

**Purpose**: Package validated basins into BasinFeedback packets with instant PreCard templates.

**Tech**:
- Rust structs + serde for JSON serialization
- TCP or UDP multicast for LAN distribution
- **PreCard template generation** (instant, no LLM)
- Tiered synthesis: Template (instant) → Light → Heavy (future)

**Output Format**: BasinFeedback v0.3 with PreCard

**Key Files**:
```
src/feedback/
  schema.rs      # BasinFeedback + PreCard structs
  precard.rs     # Template generation
  emitter.rs     # Dispatch logic
```

---

### 7. Governor (Adaptive thresholds with tempo-aware routing)
**Purpose**: PID-lite controller to maintain target basin rate and quality.

**Tech**:
- Pure Rust, simple proportional controller
- Observes: basin_rate, nd_cohesion, tempo distribution
- Adjusts: persistence_min (per-tempo), density_threshold, nd_radius
- **Tempo-aware**: Urgent packets bypass persistence checks

**Key Files**:
```
src/governor/
  controller.rs  # Threshold adjustment logic
  tempo.rs       # Tempo-specific policies
```

---

## Phase Boundaries

### Phase 1: Minimal Loop (N-D Primary)
**Goal**: Prove N-D streaming density clustering works with two-tempo decay and PreCard templates.

**Core Principles (v0.3):**
- **N-D is primary**: All basin detection in full-fidelity N-D space
- **2D is viz only**: Pure projection for human monitoring (oscilloscope)
- **No 2D physics**: No annealing/diffusion in 2D - just render N-D state
- **Two-tempo decay**: Fast (τ=2s), Slow (τ=30s), Urgent (bypass persistence)
- **Instant feedback**: PreCard templates (no LLM), log for Light/Heavy synth later

**Scope**:
- N-D streaming density clustering (micro-cell grid)
- Mock embedding service (hardcoded 768d vectors for testing)
- Minimal ledger (in-memory Vec, no VLC yet)
- Basin validator (medoid + cohesion)
- **Terminal output only** (no visualization)
- BasinFeedback with PreCard template (no network dispatch)
- Manual emission via CLI (no agent integration)

**Deliverables**:
- `sefi emit "phrase" --amp 0.8 --tempo fast` adds packet to ledger
- N-D clustering detects basins with tempo-aware persistence
- Terminal prints detected basins with medoid phrases + instant PreCard template
- Alerts (Urgent tempo) bypass persistence and trigger immediately
- Unit tests pass, demo script works

**Key Files to Build**:
```
src/
  types.rs           # ✅ DONE - Core data structures
  ledger/store.rs    # ✅ DONE - Simple in-memory version
  bin/sefi.rs        # ✅ DONE (stub) - CLI parsing

  clustering/density.rs  # TODO M1.1 - Micro-cell density grid
  clustering/tempo.rs    # TODO M1.1 - Two-tempo decay logic

  validator/medoid.rs    # TODO M1.2 - Medoid computation
  validator/cohesion.rs  # TODO M1.2 - Silhouette score

  feedback/precard.rs    # TODO M1.3 - Template generation

  engine.rs              # TODO M1.4 - Wire everything together
```

---

### Phase 2: Production Loop + Visualization
**Goal**: Full BasinFeedback pipeline with network dispatch AND visualization.

**Adds**:
- **2D Visualization** (WASM-WebGPU canvas for web/edge deployment)
- Real embedding service integration
- VLC-backed ledger with compression
- Network feedback emitter (TCP/UDP dispatch)
- Governor with tempo-aware adaptive thresholds
- Ridge and peak handling
- Light/Heavy synthesis tiers (LLM integration)

**Viz Deployment**:
- WASM build target for edge "ganglion" nodes (Jetson Nano, etc.)
- Web server option (serve canvas to browser)
- Native window option (local monitoring)

---

### Phase 3: Synthesis Integration
**Goal**: Auto-generate crux cards from basins.

**Adds**:
- Synthesizer agent (LLM integration)
- Full crux card schema (Light/Heavy synthesis)
- Steelman adversary validation
- Actuator dispatch

---

## Tech Stack (Minimal Dependencies)

```toml
[dependencies]
# GPU compute (proven by VLC)
wgpu = "26.0"
bytemuck = "1.19"
futures-intrusive = "0.5"
pollster = "0.4.0"

# Math (minimal)
nalgebra = "0.33"  # for PCA only

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async HTTP (for embedding client, Phase 2)
hyper = { version = "1.0", optional = true }

# VLC integration (Phase 2)
vlc = { path = "../vlc" }
```

**Total: ~7-8 crates** (comparable to VLC's 7)

---

## Integration with VLC

### Compression
- Ledger entries older than active window → batch compress via VLC
- Active window: last ~10K packets in memory
- Archival: VLC-compressed on disk

### Retrieval
- "Pull contributors for basin" → VLC query by rationale_hash list
- Sub-millisecond retrieval (per VLC specs)

### Projection Synergy
- VLC learns anchor points in N-D space
- Could reuse VLC anchors as PCA basis (optional future optimization)

---

## Directory Structure

```
sefi/
├── src/
│   ├── types.rs              # Core structs (ConceptPacket, BasinFeedback)
│   ├── brane/
│   │   ├── field.rs          # GPU field state
│   │   ├── kernels.wgsl      # WGSL shaders
│   │   ├── annealer.rs       # Main annealing loop
│   │   └── detector.rs       # Feature detection
│   ├── embed/
│   │   ├── client.rs         # HTTP embedding client
│   │   └── cache.rs          # LRU cache
│   ├── ledger/
│   │   ├── store.rs          # Append-only vector store
│   │   ├── window.rs         # Active memory window
│   │   └── vlc_backend.rs    # VLC compression
│   ├── validator/
│   │   ├── ndspace.rs        # N-D metrics
│   │   └── medoid.rs         # Medoid computation
│   ├── projection/
│   │   ├── pca.rs            # PCA projection
│   │   └── model.rs          # Load/apply
│   ├── feedback/
│   │   ├── schema.rs         # BasinFeedback struct
│   │   └── emitter.rs        # Network dispatch
│   ├── governor/
│   │   └── controller.rs     # Adaptive thresholds
│   └── bin/
│       └── sefi.rs           # CLI interface
├── docs/
│   ├── ARCHITECTURE.md       # This file
│   ├── SPEC.md               # Original spec
│   └── KERNELS.md            # Shader documentation
├── tests/
│   ├── brane_tests.rs
│   ├── validator_tests.rs
│   └── integration_tests.rs
├── Cargo.toml
└── README.md
```

---

## Build Strategy

### Phase 1 Milestones (v0.3 N-D Primary - No Viz)
1. **M1.1**: Streaming density clustering + **two-tempo decay** (basins form, alerts bypass)
2. **M1.2**: Basin validator (medoid + cohesion for valleys)
3. **M1.3**: Feedback output + **PreCard template** (instant feedback, no LLM)
4. **M1.4**: Wire CLI to engine (emit → cluster → feedback)
5. **M1.5**: Integration tests + demo script

**Each milestone**: Unit tests pass, CLI demo works, git commit.

**Phase 1 Success Criteria**: Working N-D density clustering with two-tempo decay, basins detected with medoid representatives, PreCard templates generated instantly, **terminal output only** (viz deferred to Phase 2).

**Foundation Complete**: ✅ types.rs, ledger/store.rs, CLI stub

---

## Open Questions for Phase 1

1. **Clustering**: Micro-cell grid parameters (cell size, decay rate)?
2. **Test data**: Generate synthetic embeddings or use real model?
3. **Persistence thresholds**: How many ticks before basin matures?

**Defaults** (use these if unsure):
- Micro-cell grid with 0.1 radius in normalized N-D space
- Synthetic (mock 768d vectors for Phase 1)
- 5 ticks minimum persistence for Slow tempo basins

**Deferred to Phase 2**:
- Viz dimensions (256×256 default)
- Projection method (PCA default)
- WASM compilation flags

---

## Next Steps

1. ✅ Architecture updated to N-D primary
2. Add two-tempo decay to SPEC.md
3. Add PreCard template to feedback schema
4. Initialize Cargo project
5. Write `types.rs` (ConceptPacket, Tempo, PreCard, BasinFeedback)
6. Implement M1.1 (ledger + emission)

---

**Architecture Status**: 📋 v0.3 N-D PRIMARY
**Core Principle**: N-D is truth, 2D is oscilloscope
**Phase 1**: Streaming density clustering with two-tempo decay + PreCard templates
**Ready to Build**: ✅

---

*N-D fidelity for semantics, 2D projection for humans* 💎

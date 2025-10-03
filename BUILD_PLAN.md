# Sefi Phase 1: Build Plan

**Version**: v0.3 (N-D Primary)
**Target**: Minimal loop with terminal output
**Estimated**: 8-12 hours total

---

## Foundation Status ✅

**Completed**:
- ✅ types.rs - Complete v0.3 schema (Tempo, PreCard, BasinFeedback)
- ✅ ledger/store.rs - In-memory ledger with unit tests
- ✅ bin/sefi.rs - CLI stub (command parsing works)

**Ready to build**: M1.1

---

## Milestone Breakdown

### M1.1: Streaming Density Clustering + Two-Tempo Decay
**Estimated**: 3-4 hours

**Files to create**:
```
src/clustering/density.rs  # Micro-cell density grid
src/clustering/basins.rs   # Basin tracking/detection
src/clustering/tempo.rs    # Two-tempo decay logic
```

**What to build**:

1. **Micro-cell density grid**:
   - Divide N-D space into cells (0.1 radius in normalized space)
   - Track density per cell (count of vectors in cell)
   - Streaming updates as new packets arrive

2. **Two-tempo decay**:
   - Fast: τ=2s (decay weight = exp(-Δt/2))
   - Slow: τ=30s (decay weight = exp(-Δt/30))
   - Urgent: no decay, bypass persistence checks

3. **Basin detection**:
   - Find local density maxima (valleys = density peaks)
   - Track persistence (# of ticks basin survives)
   - Emit basin proposal when persistence > threshold

**Tests**:
- Unit test: single packet → creates cell
- Unit test: nearby packets → same cell, density increases
- Unit test: Fast packets decay faster than Slow
- Unit test: basin detected after persistence threshold

**Success criteria**: Tests pass + basic clustering visible

---

### M1.2: Basin Validator (Medoid + Cohesion)
**Estimated**: 2-3 hours

**Files to create**:
```
src/validator/medoid.rs    # Medoid computation
src/validator/cohesion.rs  # Silhouette score
```

**What to build**:

1. **Medoid computation**:
   - Given: list of rationale_hashes (basin members)
   - Compute Fréchet mean (average of N-D vectors)
   - Find medoid = actual vector nearest to Fréchet mean
   - Return medoid's rationale_hash

2. **Cohesion (silhouette score)**:
   - For each basin member, compute:
     - a = avg distance to other members in same basin
     - b = avg distance to nearest other basin
     - silhouette = (b - a) / max(a, b)
   - Basin cohesion = avg silhouette over all members

**Tests**: Unit tests for medoid finding + cohesion computation

---

### M1.3: Feedback Emitter (PreCard Templates)
**Estimated**: 1-2 hours

**Files to create**:
```
src/feedback/precard.rs    # Template generation
```

**What to build**:

1. **PreCard template generation**:
   - Given: ValidatedBasin (medoid, contributors, cohesion)
   - Extract top-k phrases (from contributors)
   - Format summary: "Consensus on {medoid_phrase}"
   - Suggested action: "Plan spike on {medoid_phrase}"

2. **BasinFeedback assembly**: Package all metadata + PreCard

**Tests**: Unit tests for PreCard generation

---

### M1.4: Wire CLI to Engine
**Estimated**: 1-2 hours

**Files to create**:
```
src/engine.rs              # Main integration loop
```

**What to build**:

1. **Engine struct**: Owns Ledger + DensityGrid
2. **CLI integration**: Wire emit/status commands to engine
3. **Mock embeddings**: Hash-based 768d vector generation

**Tests**: Integration test for full loop

---

### M1.5: Integration Tests + Demo Script
**Estimated**: 1-2 hours

**Files to create**:
```
tests/integration_test.rs  # Full loop test
demo.sh                    # Demo script
```

**What to build**:

1. **Integration tests**: Full loop + tempo variants
2. **Demo script**: Shows basin formation with PreCards

**Success criteria**: All tests pass + demo works

---

## Phase 1 Success Criteria

**All milestones complete** ✅:
- [ ] M1.1: Clustering works
- [ ] M1.2: Validator works
- [ ] M1.3: PreCards generate
- [ ] M1.4: CLI integrated
- [ ] M1.5: Tests pass + demo works

**Terminal demo works**: `./demo.sh` shows basins forming

**All tests pass**: `cargo test --release` passes

---

## Handover for Next Session

**Start with M1.1**: Clustering implementation

**Key files already done**:
- src/types.rs
- src/ledger/store.rs
- src/bin/sefi.rs

**Key files to build**:
- src/clustering/* (M1.1)
- src/validator/* (M1.2)
- src/feedback/* (M1.3)
- src/engine.rs (M1.4)
- tests/* (M1.5)

**Estimated time**: 8-12 hours for full Phase 1

**Builder**: Start with M1.1. Tests first. 💎

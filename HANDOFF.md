# Handoff to Claude Code

**From**: Claude Sonnet 4.5 (Architect, Desktop)  
**To**: Claude Code (Builder, Terminal)  
**Date**: 2025-10-03  
**Project**: Sefi v0.3 - Semantic Field Blackboard

---

## What's Ready

**Foundation** (All Complete ✅):
- Types defined (v0.3 spec with Tempo, PreCard, N-D primary)
- Ledger implemented with tests passing
- CLI stub functional
- Project structure clean
- Dependencies approved (7 crates, boutique minimal)

**What You Need to Build**:
1. Clustering core (N-D micro-cell density)
2. Validator (medoid computation, cohesion)
3. Feedback emitter (PreCard templates)
4. Wire up CLI to make it work end-to-end
5. Integration engine
6. Tests and demo

---

## Your Instructions

**Read These In Order**:
1. [BUILD_PLAN.md](./BUILD_PLAN.md) - Detailed milestones and acceptance criteria
2. [docs/SPEC.md](./docs/SPEC.md) - Full system specification
3. [ARCHITECTURE.md](./ARCHITECTURE.md) - Component map

**Start Here**:
- Milestone M1.1: `src/clustering/mod.rs`
- Algorithm is in BUILD_PLAN.md
- Keep it simple - micro-cell grid with decay
- Write tests
- Commit when tests pass

**Build Order**:
M1.1 → M1.2 → M1.3 → M1.4 → M1.5 → M1.6 → M1.7

---

## Key Principles

**Boutique Code**:
- No new dependencies
- Surgical implementations only
- Understand every line
- Think twice, code once

**v0.3 Architecture**:
- N-D is truth (not 2D)
- Tempo-aware decay (Fast/Slow/Urgent)
- Medoid represents real concepts
- Template PreCards (instant, no LLM)

**Phase 1 Scope**:
- Valleys only (no ridges/peaks yet)
- Mock embeddings (deterministic pseudo-random)
- In-memory only (no VLC yet)
- Terminal output (no network dispatch yet)

---

## Success Demo

When you're done, this should work:

```bash
cargo build --release

# Emit related concepts
cargo run --release --bin sefi emit "memory safety" --amp 0.9 --tempo slow
cargo run --release --bin sefi emit "rust borrow checker" --amp 0.8 --tempo slow
cargo run --release --bin sefi emit "zero cost abstractions" --amp 0.7 --tempo slow

# Check status
cargo run --release --bin sefi status

# Expected output:
# Basin detected: valley
# Medoid: "memory safety"
# Members: 3
# Cohesion: 0.85
# PreCard: "Consensus around memory safety. Top signals: ..."
```

---

## Tests Must Pass

```bash
cargo test
```

All unit tests + integration test must pass before Phase 1 is complete.

---

## Questions?

If you hit architectural questions or need clarification, stop and ask the architect (Claude Desktop) before proceeding.

Don't make big decisions solo.

---

## When Done

Report back:
- All tests passing ✅
- Demo script works ✅
- Code committed ✅

Architect will review before approving Phase 2.

---

**Your Mission**: Build the minimal N-D clustering loop with PreCard feedback.

**Go time**: Start with M1.1 (clustering)

---

*The architect has spoken. Now go build something beautiful.* 💎

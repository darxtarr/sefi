# ❓ Questions That Shaped Sefi

Sefi is a living lab. Its strength comes not from having all the answers, but from asking the right questions.

This file is a running log of the best questions contributors (human or agent) have raised so far.

**Add to it. Challenge us. The questions are as valuable as the code.**

---

## Core Architecture

- **Are clusters really "valleys," or are we mixing metaphors?** Should we call them basins, peaks, or something else?

- **Why does the system need a 2-D brane at all?** Is it a decision mechanism or just a monitor?

- **What does "cooling" mean semantically?** Is forgetting = lower value, or just less persistence?

---

## Technical Feasibility

- **How do we avoid the curse of dimensionality in 768D?** Are micro-cells realistic, or should we default to HNSW/DBSCAN-lite?

- **Is the centroid of embeddings ever meaningful, or should we only trust medoids?**

- **What thresholds (persistence, depth) are adaptive, and which are fixed hyperparameters?**

---

## Use Case Anchoring

- **What's the canonical use case?** DevOps logs? Code review? Incident routing?

- **How does a BasinFeedback actually change someone's workflow?**

- **Who consumes PreCards vs Crux Cards — and why not skip straight to full synthesis?**

---

## System Health

- **What happens when urgent spikes get cooled out?**

- **How does the system handle back-pressure when too many basins form at once?**

- **Should ridges always trigger paired experiments, or sometimes just be noted as tensions?**

---

## Why This Matters

A good question isn't noise — it's a **governor adjustment**.

Keep adding them. This file should read like the Socratic history of Sefi.

---

💎 *Question everything. Build carefully.*

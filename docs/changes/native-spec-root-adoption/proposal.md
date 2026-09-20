---
adr: docs/decisions/DOCS-0001 Spec-Driven Change Lifecycle.md
status: proposed
decisions: ["Spec-Driven Change Lifecycle"]
---

# Native spec root adoption

## Why

This crate implements `rune spec`, and its own lifecycle tree sat at `docs/openspec/` behind a `spec.root` override, the one layout the deck's CORE-0019 rules out ("`rune spec` is the native implementation over `docs/`, one tree, no mirror"). The tree also failed the rune 0.5.0 caps in eleven places: two-word capabilities and changes, and three requirements over 100 words. A repository that publishes the lifecycle has to live by it.

## What Changes

- Move `docs/openspec/{changes,specs}` to `docs/{changes,specs}`, drop OpenSpec's `config.yaml` and the `spec.root` override (the repository `config.yaml` is gone, it held nothing else).
- Rename `review-integration` to `review-lane-integration`, `commit-attribution` to `commit-author-attribution`, `decision-artifacts` to `decision-artifact-records`, `development-lifecycle` to `development-lifecycle-process`, and `spec-compatibility` to `openspec-store-compatibility`, and split the three long requirements.
- Archive `openspec-store-compatibility` as abandoned: it specified `docs/openspec/` as the only writable tree, which this decision reverses.
- Adopt CORE-0019 from the deck as `DOCS-0001` through `rune adr adopt`, with `module.yaml` and an empty `defaults.yaml` so the adoption review and `rune adopt doctor` run here.

## Capabilities

### New Capabilities

- `native-spec-root`: the lifecycle tree lives at `docs/`, with three-word ids and the prose caps.

### Modified Capabilities

- `commit-author-attribution`: two requirements split at their MUST clusters, text unchanged.

## Impact

- Every path under `docs/openspec/` moves. The four decision drafts inside `development-lifecycle-process/decisions/` move with their change and stay drafts.
- `docs/changes/development-lifecycle-process/decisions/specification-and-decisions-process.md` carries an `[ISSUE]` marker from its author. It is that change's pending work, not this one's.

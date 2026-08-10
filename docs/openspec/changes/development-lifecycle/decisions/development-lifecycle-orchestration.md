---
title: "Development Lifecycle Orchestration"
description: "Deterministic coordination of Rune Deck development processes from specification to deployment."
type: adr
category: development
tags:
    - lifecycle
    - orchestration
    - dci
status: proposed
id: SPEC-0005
created: 2026-08-05
updated: 2026-08-05
author: "Rune Deck maintainers"
project: rune
related: []
responsible:
    - "Rune Deck maintainers"
accountable:
    - "Rune Deck maintainers"
consulted: []
informed: []
upstream: []
---

# Development Lifecycle Orchestration

## Context and Problem Statement

The specification and decision process, implementation and testing process, and pull-request and deployment process own distinct artifacts and approval boundaries. Substantive Rune Deck changes need one coordinator that preserves those boundaries. The coordinator must select the same phase from equivalent repository state, resume from durable evidence, and remain usable outside Rune. Trivial mechanical changes stay outside the full lifecycle.

## Decision Drivers

- Make the development lifecycle the default for substantive changes.
- Select the same phase and next action from equivalent repository state.
- Keep process approvals explicit and non-transitive.
- Resume without recreating accepted work.
- Keep skills shareable without a Rune CLI runtime dependency.
- Preserve direct user interaction across supported harnesses.

## Considered Options

1. **Independent processes**: Let every session infer how the process artifacts connect.
2. **Prose coordination**: Describe the sequence without formal state predicates.
3. **Deterministic standalone orchestration**: Coordinate the processes through explicit predicates, injected live context, and visible transitions.

## Decision Outcome

Chosen option: **Deterministic standalone orchestration**, because equivalent repository state must produce the same phase, evidence, and next action without making Rune a runtime dependency.

`DevelopmentLifecycle` coordinates the grouped process ADRs as the default path for substantive changes. The skill defines a closed phase set with entry, completion, transition, and invalidation predicates.

Ordered read-only `!` probes collect live state using standard project and platform tools. Supporting harnesses inject their output before execution; other harnesses gather equivalent context during orientation. The skill never invokes the Rune CLI.

Every phase selection reports the matched predicate, supporting evidence, and proposed transition. Valid absent states use stable sentinel output. Failed required probes stop selection rather than permitting inferred state.

Task checkpoints record user approvals and verified outcomes. Version-control history verifies that accepted artifacts remain unchanged; later changes clear the affected checkpoint and every dependent checkpoint. Resume starts at the earliest incomplete or invalid phase.

The skill uses each harness's structured question function or an equivalent bounded plain-text question. It remains in a user-facing context for approval transitions. Every approval displays the complete artifact, diff, plan, pull-request text, or outward-facing action through a full review surface.

### Consequences

- [+] Rune remains a human control plane rather than a skill runtime dependency.
- [+] The lifecycle skill can be shared independently of Rune Deck.
- [+] Equivalent state produces the same phase, evidence, and proposed transition.
- [+] Approvals apply only to the displayed transition or outward-facing action.
- [-] Missing required probe output stops the lifecycle instead of permitting a best-effort guess.
- [-] Missing version-control history leaves checkpoints unverified rather than trusted.

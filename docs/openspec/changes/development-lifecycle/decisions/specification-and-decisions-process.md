---
title: "Specification and Decisions Process"
description: "Tailored specification review and provisional decision reconciliation before implementation planning."
type: adr
category: development
tags:
    - specification
    - decisions
    - review
status: proposed
id: SPEC-0002
created: 2026-08-05
updated: 2026-08-05
# [ISSUE] use a @tag when referring to an author
author: "Rune Deck maintainers"
project: rune
# [ISSUE] use markdown links to the other related ADRs, I am sure this applies here
related: []
responsible:
    - "Rune Deck maintainers"
accountable:
    - "Rune Deck maintainers"
consulted: []
informed: []
upstream: []
---

# Specification and Decisions Process

## Context and Problem Statement

Each substantive change needs a standardized review process before implementation planning. Exploring and writing the specification may reveal further intent. Writing the ADRs may refine or introduce durable decisions. The process keeps the specification and ADR drafts aligned as the change evolves.

## Decision Drivers

- Review goals, non-goals, and scenarios against the maintainer's intent.
- Apply feedback directly to the specification under review.
- Preserve strict validation after every revision.
- Permit early decision exploration without premature acceptance.
- Keep specifications and decisions aligned before planning.

## Considered Options

1. **Sequential generic review**: Accept the specification through document-level approval, then draft ADRs.
2. **Independent drafting**: Draft specifications and ADRs independently, resolving conflicts during implementation.
3. **Tailored review with provisional decisions**: Review authored specification content through tailored questions while reconciling provisional ADR drafts before acceptance.

## Decision Outcome

Chosen option: **Tailored review with provisional decisions**, because it exposes authored intent to direct review while allowing decisions to refine the specification before implementation planning.

After strict validation, the lifecycle asks tailored questions about the authored goals, non-goals, and each requirement scenario. It uses the harness's structured question function when available and an equivalent bounded plain-text question otherwise. Every acceptance question displays the complete specification or ADR content under review.

Requested changes apply immediately. The lifecycle reruns strict validation and re-presents only affected material. A structural validation failure receives the smallest repair that preserves the maintainer's intent, followed by renewed acceptance.

Eight questions form the normal pacing boundary. If unresolved scenarios remain, the maintainer chooses whether to continue individually, review the remainder as a group, or stop for manual editing. No scenario is silently skipped.

ADR drafts may begin during specification review but remain provisional. Each specification revision reconciles the drafts. When a decision introduces newer intent, the lifecycle reopens the specification, applies that intent, and repeats strict validation and tailored review. Decision acceptance requires agreement between both artifacts.

### Consequences

- [+] Accepting a specification reflects direct review of goals, non-goals, and scenarios.
- [+] Exploring a decision can overlap review without authorizing downstream work.
- [+] Newer decision intent updates the specification instead of remaining contradictory rationale.
- [-] Every revised scenario requires validation and renewed acceptance.
- [-] Implementation planning waits until specification and decision artifacts agree.

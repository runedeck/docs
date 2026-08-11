---
title: "Implementation and Testing Process"
description: "Planning-mode handoff, approach approval, implementation, and verification recovery."
type: adr
category: development
tags:
    - implementation
    - planning
    - testing
status: proposed
id: SPEC-0003
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

# Implementation and Testing Process

## Context and Problem Statement

Accepted specifications and decisions need a deliberate implementation handoff. Planning settles the approach before files change. Execution still encounters ordinary test and validation failures that need diagnosis and repair. The process must distinguish expected debugging from scope or architecture changes that require renewed approval.

## Decision Drivers

- Separate implementation planning from execution.
- Make plan approval confirm the approach rather than grant blanket authority.
- Keep unresolved consequential decisions visible to the maintainer.
- Require verification evidence for changed behavior.
- Repair expected failures without unnecessary interruption.

## Considered Options

1. **Direct implementation**: Begin changing files from accepted specifications and ADRs without a planning handoff.
2. **Blanket plan authorization**: Treat one approved plan as authority for every later implementation action.
3. **Planning-mode handoff with bounded execution**: Approve the approach in planning mode, then execute and debug within that boundary.

## Decision Outcome

Chosen option: **Planning-mode handoff with bounded execution**, because implementation needs an approved approach while routine debugging should continue without repeated architecture review.

After specification and decision acceptance, the lifecycle enters the harness's planning mode or corresponding planning function. It creates and displays an implementation plan from those artifacts, then asks the maintainer to approve the approach.

Approval exits planning mode and permits implementation. It does not authorize scope, architecture, or publication decisions that the plan did not resolve. Those decisions receive their own tailored question when they arise.

Implementation produces test or verification evidence appropriate to the changed behavior. Failed checks trigger diagnosis and repair within the approved approach, followed by rerunning the relevant verification. A repair that changes scope or architecture returns to maintainer review.

Implementation finishes with verified changes staged and displayed for review. Staged approval confirms the diff but does not authorize pull-request publication.

### Consequences

- [+] Code and documentation changes start from an approved implementation approach.
- [+] Routine debugging proceeds without asking before every repair.
- [+] Scope and architecture changes remain explicit decisions.
- [-] Failing checks keep the lifecycle in implementation until evidence passes.
- [-] Implementation pauses when a repair exceeds the approved approach.

---
title: "Pull Request and Deployment Process"
description: "Separate review, publication, merge, release, and deployment approval boundaries."
type: adr
category: development
tags:
    - pull-request
    - review
    - deployment
status: proposed
id: SPEC-0004
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

# Pull Request and Deployment Process

## Context and Problem Statement

A verified staged diff, a published pull request, a merge, a release, and an installation are distinct external actions. The process must keep their approvals clear. Documentation changes also need a practical deployment definition because merge itself publishes the reviewed files. Deployment completion therefore depends on destination evidence, not only command success.

## Decision Drivers

- Review staged content before commit and publication.
- Keep pull-request creation separate from staged-diff approval.
- Preserve required platform review and continuous-integration checks.
- Require approval at each distinct outward-facing action.
- Define deployment through observable destination evidence.

## Considered Options

1. **Staged approval authorizes publication**: Treat staged-diff approval as authority for pull-request creation and merge.
2. **One deployment approval**: Approve every publication, merge, release, and installation through one deployment decision.
3. **Explicit action boundaries**: Keep staged review, pull-request publication, merge, release, and installation separate, combining only actions that are operationally identical.

## Decision Outcome

Chosen option: **Explicit action boundaries**, because each external action changes a different surface and earlier approval must not silently carry forward.

After implementation and verification pass, the lifecycle stages and displays the changes for maintainer review. Approval of the staged diff causes the lifecycle to prepare and display a pull-request title and body, then ask separately before publishing the pull request.

Pull-request creation does not authorize merge. Required review and continuous-integration checks remain in force. When merge itself performs deployment, one explicit merge approval covers both actions. The lifecycle displays the destination and reviewed material, performs the merge, then verifies the result at its intended destination.

For documentation, deployment is the approved merge that makes the reviewed files visible in the target GitHub repository. The lifecycle verifies that visibility without asking a duplicate deployment question.

A separate release or installation retains its own approval at execution time. Approval for an earlier publication action does not carry forward.

### Consequences

- [+] Staged review and pull-request publication remain separate decisions.
- [+] Pull-request publication and merge remain separate decisions.
- [+] Documentation merge and deployment share one approval because they are one action.
- [+] Deployment completion depends on visible destination evidence.
- [-] Releases and installations require additional approval when they are separate actions.
- [-] The lifecycle asks more questions than a bundled publication flow.

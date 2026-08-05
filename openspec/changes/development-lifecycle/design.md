## Context

See `proposal.md` for motivation and `specs/development-lifecycle/spec.md` for observable behavior.

Rune already has independent specification interfaces, decision artifacts, implementation planning practices, test commands, staged review, pull-request checks, and publication mechanisms. They do not yet form one resumable process. The deck's `development` domain is reserved for review flow, verification, commit conventions, and delivery discipline, so it owns the coordinating skill.

## Goals / Non-Goals

**Goals:**

- Make this the default Rune Deck process for substantive changes.
- Coordinate existing lifecycle functions without replacing their enforcement.
- Keep the interview portable across interactive AI harnesses.
- Preserve explicit user decisions at phase transitions.
- Resume from repository and platform state rather than conversational memory.

**Non-Goals:**

- Govern trivial mechanical changes that do not alter behavior, architecture, delivery, or shared process.
- Route RuneSpec commands through OpenSpec or OpenSpec commands through RuneSpec.
- Use the Rune CLI as a runtime dependency of the skill.
- Bind the skill to the Rune Deck layout or prevent standalone sharing.
- Replace specialized specification, ADR, testing, review, or deployment functions.
- Infer approval from validation success, a created pull request, or an earlier approval.
- Make unattended merge or deployment the default.

## Decisions

### DevelopmentLifecycle coordinates the process

The canonical skill is `DevelopmentLifecycle` under the deck's `development` domain. It inspects the active change, identifies the earliest incomplete phase, proposes the appropriate native function, and asks before starting it.

The skill coordinates judgment and sequencing. Artifact validators, version control, continuous integration, and deployment tools remain the enforcement boundaries for their own operations. Rune may install, select, or update the skill for a human operator, but the skill never invokes the Rune CLI.

A monolithic replacement command was rejected because it would duplicate those systems and couple the lifecycle to one interface.

### Skill execution is a formal phase machine

`DevelopmentLifecycle` places ordered read-only `!` probes in the canonical skill body. The commands use standard project and platform tools, never Rune. A supporting harness injects their output before execution. A harness that ignores inline injection follows the same commands during its orientation step before evaluating phase predicates.

The skill defines a closed phase set, entry predicates, completion predicates, permitted transitions, invalidation rules, and the next action for every state. The same injected state produces the same phase and proposed action. Every selection reports the matched predicate, evidence, and transition before execution.

Probe output distinguishes absence from failure. Optional context such as no active pull request returns a stable absent-state value and participates in phase selection. A required probe that cannot produce usable output stops orientation instead of allowing the model to guess.

The skill remains in a user-facing context throughout the lifecycle because its approval transitions require direct questions. Tool access is limited to the declared probes and the native tools required by the active phase.

A prose-only workflow was rejected because it leaves phase selection and recovery to interpretation. A Rune command as the state oracle was rejected because Rune is the human control plane, not a runtime dependency for shared skills.

### Approval questions include the reviewed material

An artifact-acceptance question uses a full-content review surface for the exact specification, ADR, plan, staged diff, or pull-request text. A preview is used only when it shows every reviewed line. Truncated artifacts open in the configured editor or render completely before the question. Outward-facing actions show the destination and material that will change external state.

A path, summary, or hidden-line preview is insufficient because the user cannot verify the proposed approval from it. The approval applies only to the content displayed through the complete review surface.

### The specification interview is harness-portable

The canonical skill describes a structured question capability rather than naming a provider tool. Each harness uses its corresponding structured question function. When none exists, the skill asks the same bounded question in plain text and waits.

The interview runs in the user-facing context. It first reviews goals and non-goals, then walks authored scenarios with questions derived from their actual triggers, outcomes, and boundaries. Generic approval questions do not satisfy the review.

After eight questions, unresolved scenarios cause a continuation choice. The user may continue individually, review the remainder as a group, or stop for manual editing. Eight is a pacing boundary, not a silent coverage limit.

Naming one harness's question function in the canonical skill was rejected because the deck compiles the same behavior for several harnesses.

### Interview changes apply immediately

An adjustment edits the specification, reruns strict validation, and re-presents only the affected material. A structural validation failure receives the smallest repair that preserves the user's intent. The interview remains active until the revised content validates and the user accepts it.

Collecting notes for a later edit pass was rejected because review comments can drift from the artifact they describe and leave acceptance ambiguous.

### Tasks record phase acceptance

`tasks.md` carries explicit lifecycle checkpoints, including completion of the specification interview, decision review, implementation-plan review, verification, staged review, pull-request review, and deployment.

The lifecycle checks a task only after the corresponding user approval or verified outcome. On orientation and resume, version-control history compares accepted artifacts with their checkpoint. Any later change clears that phase and every dependent checkpoint before continuing.

No separate review ledger is introduced. This keeps the workflow readable in plain Markdown. Version-control comparison supplies the stale-checkpoint detection that the checkbox alone cannot provide.

A dedicated review artifact was rejected in favor of the lighter tasks checkpoint selected for this process.

### Phase transitions preserve approval boundaries

The lifecycle order is:

```text
specification
    -> tailored specification review
    -> decision artifacts
    -> implementation plan
    -> implementation and testing
    -> staged review and pull request
    -> deployment
```

Decision drafting may overlap specification review as exploratory work, but decision acceptance remains after specification acceptance. Specification revisions require provisional decisions to be reconciled before review.

After decision acceptance, the lifecycle enters the harness's planning mode or corresponding planning function. Plan approval confirms the implementation approach, then implementation starts outside planning mode. New consequential decisions still require user verification when they arise.

Failed verification is normal implementation feedback. The lifecycle diagnoses and fixes failures within the approved approach, then reruns the relevant checks. A fix that changes scope or architecture returns to user verification.

Validation success permits an approval question; it never answers one. Plan approval does not authorize implementation publication, staged-change approval does not authorize pull-request creation, pull-request creation does not authorize merge, and merge approval does not authorize a separate release or installation.

A decision draft that introduces newer intent reopens the specification. The lifecycle applies that intent to the requirements and scenarios, reruns validation and tailored review, and keeps the decision provisional until both artifacts agree. A code or test finding returns the lifecycle to implementation. Pull-request feedback returns it to the affected phase rather than being patched outside the artifact chain.

### Deployment means published availability

Deployment is the point at which the reviewed result reaches its intended destination. For software this may be a release or installation. For documentation it is the merge that makes the files visible in the target GitHub repository.

When merge itself performs deployment, one explicit merge approval covers that action; the lifecycle does not ask again for the same publication. The skill verifies destination-specific evidence before checking deployment complete. A separate release or installation retains its own approval boundary.

### Resume derives state from durable systems

The skill reconstructs progress from OpenSpec artifacts, lifecycle checkboxes, decision records, version-control state, test evidence, pull-request state, and deployment evidence. It does not rely on one harness transcript.

On resume, it validates completed phases in order and starts at the earliest incomplete or invalid phase. It reuses accepted artifacts rather than recreating them.

## Risks / Trade-offs

- Checkpoint validation depends on sufficient version-control history. Missing or ambiguous history leaves the phase unverified rather than trusting the checkbox.
- A cross-harness interview cannot guarantee identical presentation. The behavioral contract requires equivalent questions and outcomes, not identical UI.
- One coordinating skill can grow too broad. Phase mechanics stay in focused skills and commands, with this skill limited to routing and approvals.
- Deployment evidence differs by destination. Each integration defines what published availability means and how it is verified.

## Migration Plan

- Accept and implement the decision-artifacts capability as the specification-to-ADR boundary.
- Add lifecycle checkpoint sections to the project tasks template and workflow schema.
- Author `DevelopmentLifecycle` in the deck's `development` domain with portable question guidance.
- Connect the skill to existing specification, decision, planning, testing, review, and deployment functions without wrapping one specification CLI in the other.
- Add fixtures for accepted, revised, interrupted, review-returned, and deployed changes.
- Exercise the lifecycle on a documentation-only change and a software change before making it a default deck capability.
